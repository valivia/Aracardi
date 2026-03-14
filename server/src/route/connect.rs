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
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{debug, info};

use crate::{
    AppState,
    structs::{
        game::{Game, client::Client},
        protocol::{game_update::GameUpdate, message::OutgoingMessage},
    },
};

const PING_INTERVAL: Duration = Duration::from_secs(15);
const PONG_TIMEOUT: Duration = Duration::from_secs(10);
const CONNECTION_TIMEOUT: Duration = Duration::from_mins(5);

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, game_id))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>, game_id: String) {
    // Drop connection if game doesn't exist
    if !state.games.contains_key(&game_id) {
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: 1011,
                reason: "Game not found".into(),
            })))
            .await;
        return;
    }

    let (tx, rx) = mpsc::channel::<Message>(32);

    // Create or reconnect player
    let client_id =
        match Client::authenticate(&mut socket, &state, tx.clone(), game_id.clone()).await {
            Ok(id) => id,
            Err(_) => return, // authentication failed
        };

    let is_host = match state.games.get(&game_id) {
        Some(game) => game.host_id == client_id,
        None => return, // game was deleted during authentication
    };

    if is_host {
        if let Some(game) = state.games.get(&game_id) {
            game.send_host_status()
        };
    }

    let client_type = if is_host { "host" } else { "player" };

    info!("[game] {game_id} | {client_type} {client_id} connected");

    let (sender, mut receiver) = socket.split();

    // Ping task
    let ping_task = tokio::spawn(ping_task(
        tx.clone(),
        state.clone(),
        game_id.clone(),
        client_id.clone(),
    ));

    let send_task = tokio::spawn(send_task(rx, sender, game_id.clone(), client_id.clone()));

    // Receive loop
    while let Some(result) = receiver.next().await {
        match result {
            Err(e) => {
                debug!("[game] {game_id} | receive error for {client_id}: {e}");
                break;
            }
            Ok(msg) => {
                match msg {
                    Message::Close(_) => {
                        break;
                    }
                    Message::Pong(_) => {
                        // debug!("Player: {client_id} pong, game: {game_id}");
                        if let Some(mut game) = state.games.get_mut(&game_id) {
                            if let Some(client) = game.clients.get_mut(&client_id) {
                                client.last_seen = std::time::Instant::now();
                            }
                        }
                    }
                    Message::Text(text) => {
                        Game::on_message(state.clone(), &game_id, &client_id, &text).await
                    }
                    _ => {}
                }
            }
        }
    }

    debug!("[game] {game_id} | connection lost to {client_type} {client_id}");

    // Drop client
    if let Some(mut game) = state.games.get_mut(&game_id) {
        game.remove_client(&client_id);
    }

    // Communicate host exit to clients
    if let Some(game) = state.games.get(&game_id) {
        game.send_host_status()
    };

    // Stop tasks
    drop(tx);
    let _ = send_task.await;
    ping_task.abort();

    if is_host {
        // Wait a while to give host a chance to reconnect
        tokio::time::sleep(CONNECTION_TIMEOUT).await;

        // Drop if host hasn't reconnected
        let removed = state
            .games
            .remove_if(&game_id, |_, game| !game.is_host_connected());

        if removed.is_some() {
            info!("[game] {game_id} | deleted after inactivity");
        }
    }
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

    debug!("[game] {game_id} | Closed send thread for {client_id}");
}

pub async fn ping_task(
    tx: Sender<Message>,
    state: Arc<AppState>,
    game_id: String,
    client_id: String,
) {
    let mut interval = tokio::time::interval(PING_INTERVAL);
    interval.tick().await; // skip immediate first tick

    loop {
        if tx.send(Message::Ping(Bytes::new())).await.is_err() {
            break; // channel closed, connection is gone
        }

        interval.tick().await;

        let should_drop = {
            state
                .games
                .get(&game_id)
                .and_then(|game| {
                    game.clients
                        .get(&client_id)
                        .map(|client| client.last_seen.elapsed() > PING_INTERVAL + PONG_TIMEOUT)
                })
                .unwrap_or(true) // client already gone, exit task
        };

        if should_drop {
            if let Some(mut game) = state.games.get_mut(&game_id) {
                game.remove_client(&client_id);
            }
            break;
        }
    }

    let _ = tx
        .send(Message::Close(Some(CloseFrame {
            code: 1001,
            reason: "Game closed".into(),
        })))
        .await;

    debug!("[game] {game_id} | Closed ping thread for {client_id}");
}
