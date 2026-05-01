use axum::{
    extract::ws::{Message, WebSocket},
    http::HeaderMap,
};
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self},
    oneshot,
};
use tracing::{debug, info};

use crate::{
    AppState,
    structs::{
        game::{GameEndReason, MAX_HOST_ABSENCE, client::Client},
        protocol::connection::{CloseReason, DisconnectReason},
    },
};

use super::heartbeat::ping_task;
use super::receive::receive;
use super::send::send_task;

pub async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    game_join_id: String,
    headers: HeaderMap,
) {
    if !state.games.contains_key(&game_join_id) {
        let _ = socket.send(CloseReason::NotFound.to_message()).await;
        return;
    }

    let (client_tx, client_rx) = mpsc::channel::<Message>(32);

    let client_id = match Client::authenticate(
        &mut socket,
        &state,
        &headers,
        client_tx.clone(),
        &game_join_id,
    )
    .await
    {
        Ok(id) => id,
        Err(auth_error) => {
            if let Some(close_message) = auth_error.into_connection_close() {
                let _ = socket.send(close_message.to_message()).await;
            }
            return;
        }
    };

    let is_host = match state.games.get(&game_join_id) {
        Some(game) => game.host_id == client_id,
        None => return,
    };

    let (socket_tx, socket_rx) = socket.split();
    let (timeout_tx, timeout_rx) = oneshot::channel::<()>();

    // Heartbeat loop;
    let ping_task = tokio::spawn(ping_task(
        timeout_tx,
        state.clone(),
        game_join_id.clone(),
        client_id.clone(),
    ));

    // Send loop
    let send_task = tokio::spawn(send_task(client_rx, socket_tx));

    // Receive loop
    let disconnect_reason = receive(&state, &game_join_id, &client_id, socket_rx, timeout_rx).await;

    debug!("[game] {game_join_id} | {client_id} closed receive loop");

    ping_task.abort();
    let _ = ping_task.await;
    debug!("[game] {game_join_id} | {client_id} closed ping thread");

    if let Some(mut game) = state.games.get_mut(&game_join_id) {
        game.disconnect_client(&client_id, disconnect_reason.clone());
    }

    drop(client_tx);

    let _ = send_task.await;
    debug!("[game] {game_join_id} | {client_id} closed send thread");

    if is_host {
        if disconnect_reason != DisconnectReason::ClosedByClient {
            tokio::time::sleep(MAX_HOST_ABSENCE).await;
        }

        let removed = state
            .games
            .remove_if(&game_join_id, |_, game| !game.is_host_connected());

        match removed {
            Some((_id, mut game)) => {
                game.close(GameEndReason::HostLeft);
                info!(
                    "[game] {game_join_id} | Deleted game after {}",
                    if disconnect_reason == DisconnectReason::ClosedByClient {
                        "closed by host"
                    } else {
                        "host timeout"
                    }
                );
            }
            None => {
                debug!(
                    "[game] {game_join_id} | Game retained (host reconnected or already removed)"
                );
            }
        }
    }

    debug!("[game] {game_join_id} | {client_id} thread exiting");
}
