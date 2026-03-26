use axum::{
    body::Bytes,
    extract::{
        Path, State,
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};
use tracing::{debug, info};

use crate::{
    AppState,
    structs::game::{Game, client::Client},
};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const PONG_TIMEOUT: Duration = Duration::from_secs(8);
#[cfg(debug_assertions)]
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
#[cfg(not(debug_assertions))]
const CONNECTION_TIMEOUT: Duration = Duration::from_mins(5);

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, game_id.to_uppercase()))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>, game_join_id: String) {
    if !state.games.contains_key(&game_join_id) {
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: 1011,
                reason: "Game not found".into(),
            })))
            .await;
        return;
    }

    let (client_tx, client_rx) = mpsc::channel::<Message>(32);

    let client_id =
        match Client::authenticate(&mut socket, &state, client_tx.clone(), game_join_id.clone())
            .await
        {
            Ok(id) => id,
            Err(_) => return,
        };

    let is_host = match state.games.get(&game_join_id) {
        Some(game) => game.host_id == client_id,
        None => return,
    };

    if is_host {
        if let Some(game) = state.games.get(&game_join_id) {
            game.send_host_status()
        };
    }

    let client_type = if is_host { "host" } else { "player" };
    info!("[game] {game_join_id} | {client_type} {client_id} connected");

    // Main loop
    run_client(
        socket,
        state.clone(),
        game_join_id.clone(),
        client_id.clone(),
        client_rx,
        client_tx,
    )
    .await;

    // Communicate host exit to remaining clients
    if let Some(game) = state.games.get(&game_join_id) {
        game.send_host_status()
    };

    if is_host {
        tokio::time::sleep(CONNECTION_TIMEOUT).await;

        let removed = state
            .games
            .remove_if(&game_join_id, |_, game| !game.is_host_connected());

        match removed {
            Some((_id, game)) => {
                game.log_end(&state.telemetry);
                info!("[game] {game_join_id} | deleted game after host timeout");
            }
            None => {
                debug!(
                    "[game] {game_join_id} | game retained (host reconnected or already removed)"
                );
            }
        }
    }

    info!("[game] {game_join_id} | {client_id} disconnected");
}

async fn run_client(
    socket: WebSocket,
    state: Arc<AppState>,
    game_join_id: String,
    client_id: String,
    rx: Receiver<Message>,
    tx: Sender<Message>,
) {
    let (sender, mut receiver) = socket.split();

    let (timeout_tx, mut timeout_rx) = oneshot::channel::<()>();

    let ping_task = tokio::spawn(ping_task(
        tx.clone(),
        timeout_tx,
        state.clone(),
        game_join_id.clone(),
        client_id.clone(),
    ));

    let send_task = tokio::spawn(send_task(
        rx,
        sender,
        game_join_id.clone(),
        client_id.clone(),
    ));

    // Receive loop
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    None => {
                        debug!("[game] {game_join_id} | {client_id} stream ended for");
                        break;
                    }
                    Some(Err(e)) => {
                        debug!("[game] {game_join_id} | {client_id} received error: {e}");
                        break;
                    }
                    Some(Ok(msg)) => match msg {
                        Message::Close(_) => break,
                        Message::Pong(_) => {
                            if let Some(mut game) = state.games.get_mut(&game_join_id) {
                                if let Some(client) = game.clients.get_mut(&client_id) {
                                    client.last_seen = std::time::Instant::now();
                                }
                            }
                        }
                        Message::Text(text) => {
                            Game::on_message(state.clone(), &game_join_id, &client_id, &text).await;
                        }
                        _ => {}
                    },
                }
            }

            _ = &mut timeout_rx => {
                break;
            }
        }
    }

    debug!("[game] {game_join_id} | {client_id} closed receive loop");

    ping_task.abort();
    let _ = ping_task.await;
    debug!("[game] {game_join_id} | {client_id} closed ping thread");

    if let Some(mut game) = state.games.get_mut(&game_join_id) {
        debug!("[game] {game_join_id} | {client_id} client removed from game");
        game.remove_client(&client_id);
    }

    drop(tx);

    let _ = send_task.await;
    debug!("[game] {game_join_id} | {client_id} closed send thread");
}

pub async fn send_task(
    mut rx: Receiver<Message>,
    mut sender: SplitSink<WebSocket, Message>,
    game_id: String,
    client_id: String,
) {
    while let Some(msg) = rx.recv().await {
        if sender.send(msg).await.is_err() {
            break;
        }
    }
}

pub async fn ping_task(
    client_tx: Sender<Message>,
    timeout_tx: oneshot::Sender<()>,
    state: Arc<AppState>,
    game_id: String,
    client_id: String,
) {
    let mut interval = tokio::time::interval(PING_INTERVAL);
    interval.tick().await; // skip immediate first tick

    loop {
        let ping_sent_at = Instant::now();

        if client_tx.send(Message::Ping(Bytes::new())).await.is_err() {
            break;
        }

        tokio::time::sleep(PONG_TIMEOUT).await;

        let timed_out = state
            .games
            .get(&game_id)
            .and_then(|game| {
                game.clients
                    .get(&client_id)
                    .map(|client| client.last_seen < ping_sent_at)
            })
            .unwrap_or(true); // if game/client gone, consider timed out

        if timed_out {
            break;
        }

        interval.tick().await;
    }

    let _ = timeout_tx.send(());
}
