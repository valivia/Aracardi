use std::{sync::Arc, time::Duration};

use axum::extract::ws::{Message, WebSocket};
use nanoid::nanoid;
use tokio::{sync::mpsc, time::timeout};
use tracing::warn;

use crate::structs::{
    app_state::AppState,
    protocol::message::{ConnectionClose, IncomingMessage, OutgoingMessage},
};

pub type Tx = mpsc::Sender<Message>;

pub const CLIENT_ID_LENGTH: usize = 10;
pub type ClientId = String;

pub enum AuthError {
    GameNotFound,
    GameFull,
    TimedOut,

    SendFailed,
    ClientDisconnected,
}

impl AuthError {
    pub fn into_connection_close(&self) -> Option<ConnectionClose> {
        match self {
            AuthError::GameNotFound => Some(ConnectionClose::NotFound),
            AuthError::TimedOut => Some(ConnectionClose::TimedOut),
            AuthError::GameFull => Some(ConnectionClose::GameFull),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    pub tx: Tx,
    pub last_seen: std::time::Instant,
}

impl Client {
    pub fn new(tx: Tx) -> Self {
        Client {
            tx,
            last_seen: std::time::Instant::now(),
        }
    }

    pub fn generate_id() -> ClientId {
        nanoid!(CLIENT_ID_LENGTH, &nanoid::alphabet::SAFE)
    }

    pub fn send(&self, message: Message) {
        self.tx.try_send(message).ok();
    }

    pub async fn authenticate(
        socket: &mut WebSocket,
        state: &Arc<AppState>,
        tx: Tx,
        game_id: &str,
    ) -> Result<ClientId, AuthError> {
        let client_id = timeout(
            Duration::from_secs(10),
            Self::handshake(socket, state, tx, game_id),
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
                    return Self::resolve_client(state, tx, game_id, requested_id).await;
                }
                Err(e) => warn!("Parse error: {e}"),
                _ => warn!("Unexpected message type during connect"),
            }
        }
    }

    async fn resolve_client(
        state: &Arc<AppState>,
        tx: Tx,
        game_id: &str,
        requested_id: String,
    ) -> Result<ClientId, AuthError> {
        let mut game = match state.games.get_mut(game_id) {
            Some(g) => g,
            None => {
                return Err(AuthError::GameNotFound);
            }
        };

        let client_id = requested_id
            .len()
            .eq(&CLIENT_ID_LENGTH)
            .then_some(requested_id);

        Ok(game.upsert_client(Client::new(tx), client_id))
    }
}
