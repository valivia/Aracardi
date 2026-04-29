use std::{sync::Arc, time::Duration};

use axum::{
    extract::ws::{Message, WebSocket},
    http::HeaderMap,
};
use tokio::time::timeout;
use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{
        client::{Client, Tx, connection::ClientConnection, socket::ClientSocket},
        state::ClientId,
    },
    protocol::{
        connection::CloseReason,
        message::{incoming::IncomingMessage, outgoing::OutgoingMessage},
    },
};

pub enum AuthError {
    GameNotFound,
    GameFull,
    TimedOut,

    SendFailed,
    ClientDisconnected,
}

impl AuthError {
    pub fn into_connection_close(&self) -> Option<CloseReason> {
        match self {
            AuthError::GameNotFound => Some(CloseReason::NotFound),
            AuthError::TimedOut => Some(CloseReason::TimedOut),
            AuthError::GameFull => Some(CloseReason::GameFull),
            _ => None,
        }
    }
}

impl Client {
    pub async fn authenticate(
        socket: &mut WebSocket,
        state: &Arc<AppState>,
        headers: &HeaderMap,
        tx: Tx,
        game_id: &str,
    ) -> Result<ClientId, AuthError> {
        let client_id = timeout(
            Duration::from_secs(10),
            Self::handshake(socket, state, headers, tx, game_id),
        )
        .await
        .unwrap_or(Err(AuthError::TimedOut))?;

        socket
            .send(OutgoingMessage::ClientId(client_id.clone()).to_message())
            .await
            .map_err(|_| AuthError::SendFailed)?;

        Ok(client_id)
    }

    async fn handshake(
        socket: &mut WebSocket,
        state: &Arc<AppState>,
        headers: &HeaderMap,
        tx: Tx,
        game_id: &str,
    ) -> Result<ClientId, AuthError> {
        loop {
            let msg = match socket.recv().await {
                Some(Ok(msg)) => msg,
                _ => return Err(AuthError::ClientDisconnected),
            };

            let Message::Text(text) = msg else {
                continue;
            };

            match IncomingMessage::parse_message(&text) {
                Ok(IncomingMessage::Connect(requested_id)) => {
                    return Self::resolve_client(
                        state,
                        game_id,
                        requested_id,
                        ClientConnection::new(headers),
                        ClientSocket::new(tx),
                    )
                    .await;
                }
                Err(e) => warn!("Parse error: {e}"),
                _ => warn!("Unexpected message type during connect"),
            }
        }
    }

    async fn resolve_client(
        state: &Arc<AppState>,
        game_id: &str,
        requested_id: Option<ClientId>,
        connection: ClientConnection,
        socket: ClientSocket,
    ) -> Result<ClientId, AuthError> {
        let mut game = match state.games.get_mut(game_id) {
            Some(g) => g,
            None => {
                return Err(AuthError::GameNotFound);
            }
        };

        if game.is_full() {
            return Err(AuthError::GameFull);
        }

        Ok(game.upsert_client(requested_id, connection, socket))
    }
}
