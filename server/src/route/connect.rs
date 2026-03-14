use axum::{
    body::Bytes,
    extract::{
        Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;

use crate::{
    AppState,
    structs::game::{Game, client::Client},
};

const PING_INTERVAL: Duration = Duration::from_secs(15);
const PONG_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response {
    if !state.games.contains_key(&game_id) {
        return Response::builder()
            .status(404)
            .body("Game not found".into())
            .unwrap();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state, game_id))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>, game_id: String) {
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // Create or reconnect player
    let client_id =
        match Client::authenticate(&mut socket, &state, tx.clone(), game_id.clone()).await {
            Ok(id) => id,
            Err(_) => return, // authentication failed
        };

    let (mut sender, mut receiver) = socket.split();

    // Ping task
    let ping_tx = tx.clone();
    let ping_state = state.clone();
    let ping_client_id = client_id.clone();
    let ping_game_id = game_id.clone();
    let ping_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(PING_INTERVAL);
        interval.tick().await; // skip immediate first tick

        loop {
            interval.tick().await;

            // Check if last pong is too old (missed previous ping)
            let should_drop = {
                ping_state
                    .games
                    .get(&ping_game_id)
                    .and_then(|game| {
                        game.clients
                            .get(&ping_client_id)
                            .map(|client| client.last_seen.elapsed() > PING_INTERVAL + PONG_TIMEOUT)
                    })
                    .unwrap_or(true) // client already gone, exit task
            };

            if should_drop {
                if let Some(mut game) = ping_state.games.get_mut(&ping_game_id) {
                    game.remove_client(&ping_client_id);
                }
                break;
            }

            if ping_tx.try_send(Message::Ping(Bytes::new())).is_err() {
                break; // sender closed
            }
        }
    });

    // Send task
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Receive loop
    while let Some(Ok(msg)) = receiver.next().await {
        Game::on_message(state.clone(), &game_id, &client_id, msg).await;
    }

    send_task.abort();
    ping_task.abort();

    if let Some(mut game) = state.games.get_mut(&game_id) {
        game.remove_client(&client_id);
    }
}
