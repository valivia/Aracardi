use axum::{body::Bytes, extract::ws::Message, http::HeaderMap};
use serde::Serialize;
use tokio::{sync::mpsc, time::Instant};
use tracing::info;
use uuid::Uuid;

use crate::structs::game::client::{connection::ConnectionMeta, session::SessionMeta};

pub mod connection;
pub mod handshake;
pub mod session;

pub type Tx = mpsc::Sender<Message>;

pub type ClientId = Uuid;

#[derive(Clone, Serialize)]
pub struct Client {
    pub id: ClientId,
    pub session: SessionMeta,
    #[serde(skip)]
    pub connection: Option<ConnectionMeta>,
}

impl Client {
    pub fn new(tx: Tx, headers: &HeaderMap) -> Self {
        Client {
            id: Client::generate_id(),
            connection: Some(ConnectionMeta::new(tx)),
            session: SessionMeta::new(headers),
        }
    }

    pub fn generate_id() -> ClientId {
        Uuid::new_v4()
    }

    pub fn send(&self, message: Message) {
        if let Some(connection) = &self.connection {
            connection.try_send(message);
        }
    }

    pub fn ping(&mut self) -> Result<(), ()> {
        let Some(connection) = &mut self.connection else {
            return Err(());
        };

        connection.last_ping_at = Instant::now();
        connection
            .tx
            .try_send(Message::Ping(Bytes::new()))
            .map_err(|_| ())?;

        Ok(())
    }

    pub fn pong(&mut self) {
        if let Some(connection) = &mut self.connection {
            connection.last_seen_at = Instant::now();
        }
    }

    pub fn is_ping_timed_out(&self) -> bool {
        let Some(connection) = &self.connection else {
            return false;
        };

        return connection.last_seen_at < connection.last_ping_at;
    }
}
