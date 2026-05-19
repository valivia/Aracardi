use axum::{body::Bytes, extract::ws::Message};
use serde::Serialize;
use tokio::{sync::mpsc, time::Instant};
use uuid::Uuid;

use crate::structs::{
    game::client::{
        connection::ClientConnection, preferences::ClientPreferences, session::SessionData,
        socket::ClientSocket,
    },
    protocol::connection::{CloseReason, DisconnectReason},
};

pub mod connection;
pub mod handshake;
pub mod preferences;
pub mod session;
pub mod socket;

pub type Tx = mpsc::Sender<Message>;

pub type ClientId = Uuid;

#[derive(Clone, Serialize)]
pub struct Client {
    #[serde(serialize_with = "uuid::serde::hyphenated::serialize")]
    pub id: ClientId,
    pub is_host: bool,
    pub session: SessionData,
    pub preferences: Option<ClientPreferences>,
    pub connection: ClientConnection,
    #[serde(skip)]
    pub socket: Option<ClientSocket>,
}

impl Client {
    pub fn new(id: ClientId, connection: ClientConnection, socket: ClientSocket) -> Self {
        Client {
            id,
            is_host: false,
            connection,
            preferences: None,
            session: SessionData::default(),
            socket: Some(socket),
        }
    }

    pub fn generate_id() -> ClientId {
        Uuid::new_v4()
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_some()
    }

    pub fn send(&self, message: Message) {
        if let Some(socket) = &self.socket {
            socket.try_send(message);
        }
    }

    pub fn reconnect(&mut self, socket: ClientSocket) {
        self.socket = Some(socket);
        self.session.log_reconnect();
    }

    pub fn disconnect(&mut self, reason: DisconnectReason) {
        let Some(socket) = &self.socket else {
            return;
        };

        if let Some(message) = reason.to_message() {
            socket.try_send(message);
        }

        self.socket = None;
        self.session
            .log_disconnect(reason != DisconnectReason::Close(CloseReason::GameEnded));
    }

    pub fn ping(&mut self, session_tx: &Tx) -> Result<(), ()> {
        let Some(socket) = &mut self.socket else {
            return Err(());
        };

        if !socket.tx.same_channel(session_tx) {
            return Err(());
        }

        socket.last_ping_at = Instant::now();
        socket
            .tx
            .try_send(Message::Ping(Bytes::new()))
            .map_err(|_| ())?;

        Ok(())
    }

    pub fn pong(&mut self) {
        if let Some(socket) = &mut self.socket {
            socket.last_seen_at = Instant::now();
        }
    }

    pub fn is_ping_timed_out(&self) -> bool {
        let Some(socket) = &self.socket else {
            return true;
        };

        return socket.last_seen_at < socket.last_ping_at;
    }
}

pub enum ClientDisconnectError {
    NotFound,
    AlreadyDisconnected,
}
