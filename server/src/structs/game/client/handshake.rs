use std::{sync::Arc, time::Duration};

use axum::{
    extract::ws::{Message, WebSocket},
    http::HeaderMap,
};
use tokio::time::timeout;
use tracing::warn;

use crate::{
    SERVER_VERSION,
    structs::{
        app_state::AppState,
        game::{
            client::{Client, Tx, connection::ClientConnection, socket::ClientSocket},
            state::ClientId,
        },
        protocol::{
            connection::CloseReason,
            message::{incoming::IncomingMessage, outgoing::OutgoingMessage},
        },
    },
};

pub enum AuthError {
    GameNotFound,
    GameFull,
    TimedOut,

    VersionMismatch,
    ProtocolError,

    SendFailed,
    ClientDisconnected,
}

impl AuthError {
    pub fn into_connection_close(&self) -> Option<CloseReason> {
        match self {
            AuthError::GameNotFound => Some(CloseReason::NotFound),
            AuthError::TimedOut => Some(CloseReason::TimedOut),
            AuthError::GameFull => Some(CloseReason::GameFull),
            AuthError::VersionMismatch => Some(CloseReason::VersionMismatch),
            AuthError::ProtocolError => Some(CloseReason::InvalidHandshake),
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
        join_code: &str,
    ) -> Result<ClientId, AuthError> {
        let client_id = timeout(
            Duration::from_secs(10),
            Self::handshake(socket, state, headers, tx, join_code),
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
        join_code: &str,
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
                Ok(IncomingMessage::Connect(data)) => {
                    return Self::resolve_client(
                        state,
                        join_code,
                        data.client_id,
                        ClientConnection::new(data.version, headers),
                        ClientSocket::new(tx),
                    )
                    .await;
                }
                Err(e) => {
                    warn!("Parse error: {e}");
                    return Err(AuthError::ProtocolError);
                }
                _ => warn!("Unexpected message type during connect"),
            }
        }
    }

    async fn resolve_client(
        state: &Arc<AppState>,
        join_code: &str,
        requested_id: Option<ClientId>,
        connection: ClientConnection,
        socket: ClientSocket,
    ) -> Result<ClientId, AuthError> {
        // Check version
        if connection.version.major != SERVER_VERSION.major {
            return Err(AuthError::VersionMismatch);
        }

        // Get game
        let mut game = match state.games.get_mut(join_code) {
            Some(g) => g,
            None => {
                return Err(AuthError::GameNotFound);
            }
        };

        // Game state
        if game.is_full() {
            return Err(AuthError::GameFull);
        }

        Ok(game.upsert_client(requested_id, connection, socket))
    }
}
