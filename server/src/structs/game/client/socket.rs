use axum::extract::ws::Message;
use tokio::time::Instant;
use uuid::Uuid;

use crate::structs::game::client::Tx;

#[derive(Clone)]
pub struct ClientSocket {
    pub tx: Tx,

    pub last_seen_at: Instant,
    pub last_ping_at: Instant,

    pub session_id: Uuid,
}

impl ClientSocket {
    pub fn new(tx: Tx) -> Self {
        Self {
            tx,

            last_ping_at: Instant::now(),
            last_seen_at: Instant::now(),

            session_id: Uuid::now_v7(),
        }
    }

    pub fn try_send(&self, message: Message) {
        self.tx.try_send(message).ok();
    }
}
