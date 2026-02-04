use axum::{
    extract::{
        Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{AppState, structs::player::PLAYER_ID_LENGTH};

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
    let player_id = loop {
        let msg = match socket.recv().await {
            Some(Ok(msg)) => msg,
            _ => return, // client disconnected
        };

        match msg {
            Message::Text(text) if text.starts_with("connect:") => {
                let requested_id = text.trim_start_matches("connect:").to_string();

                let mut game = match state.games.get_mut(&game_id) {
                    Some(g) => g,
                    None => {
                        let _ = socket.close().await;
                        return;
                    }
                };

                // Reconnect if valid + exists
                if requested_id.len() == PLAYER_ID_LENGTH && game.player(&requested_id).is_some() {
                    game.update_player(&requested_id, tx.clone());
                    break requested_id.to_string();
                }

                // Otherwise create new player
                let new_id = game.add_player(tx.clone());
                break new_id;
            }

            // Ignore everything else until connect
            _ => continue,
        }
    };

    // Send assigned player ID
    socket
        .send(format!("player_id:{}", player_id).into())
        .await
        .unwrap();


    // Send game state
    

    let (mut sender, mut receiver) = socket.split();

    // Send task
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Receive loop
    let state_clone = state.clone();
    let game_id_clone = game_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                msg => {
                    if let Some(game) = state_clone.games.get(&game_id_clone) {
                        game.broadcast(msg);
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // Cleanup
    // if let Some(mut game) = state.games.get_mut(&game_id) {
    //     let host_id = game.host().cloned();
    //     game.remove_player(&player_id);

    //     if game.is_empty() || host_id == Some(player_id) {
    //         drop(game);
    //         state.games.remove(&game_id);
    //         println!("Game {} removed", game_id);
    //     }
    // }

    send_task.abort();
}
