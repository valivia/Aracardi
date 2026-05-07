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
    AppState, CONNECTED_CLIENTS,
    structs::{
        game::{GameEndReason, client::Client},
        protocol::connection::CloseReason,
    },
};

use super::heartbeat::ping_task;
use super::receive::receive;
use super::send::send_task;

struct ClientGuard();

impl Drop for ClientGuard {
    fn drop(&mut self) {
        CONNECTED_CLIENTS.dec();
    }
}

pub async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    join_code: String,
    headers: HeaderMap,
) {
    CONNECTED_CLIENTS.inc();
    let _guard = ClientGuard();

    if !state.games.contains_key(&join_code) {
        let _ = socket.send(CloseReason::NotFound.to_message()).await;
        return;
    }

    let (client_tx, client_rx) = mpsc::channel::<Message>(32);

    let client_id =
        match Client::authenticate(&mut socket, &state, &headers, client_tx.clone(), &join_code)
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

    let is_host = match state.games.get(&join_code) {
        Some(game) => game.host_id == client_id,
        None => return,
    };

    let (socket_tx, socket_rx) = socket.split();
    let (timeout_tx, timeout_rx) = oneshot::channel::<()>();

    // Heartbeat loop;
    let ping_task = tokio::spawn(ping_task(
        timeout_tx,
        state.clone(),
        join_code.clone(),
        client_id.clone(),
    ));

    // Send loop
    let send_task = tokio::spawn(send_task(client_rx, socket_tx));

    // Receive loop
    let disconnect_reason = receive(&state, &join_code, &client_id, socket_rx, timeout_rx).await;

    debug!("[game] {join_code} | {client_id} closed receive loop");

    ping_task.abort();
    let _ = ping_task.await;
    debug!("[game] {join_code} | {client_id} closed ping thread");

    if let Some(mut game) = state.games.get_mut(&join_code) {
        game.disconnect_client(&client_id, disconnect_reason.clone());
    }

    drop(client_tx);

    let _ = send_task.await;
    debug!("[game] {join_code} | {client_id} closed send thread");

    // Delete immediately if graceful shutdown
    if is_host && disconnect_reason.is_intentional() {
        state.remove_game(&join_code, GameEndReason::Closed);
    }
}
