use axum::extract::ws::Message;
use tokio::time::Instant;

use crate::structs::game::client::Tx;

#[derive(Clone)]
pub struct ConnectionMeta {
    pub tx: Tx,
    pub last_seen_at: Instant,
    pub last_ping_at: Instant,
    pub connected_at: Instant,
}

impl ConnectionMeta {
    pub fn new(tx: Tx) -> Self {
        Self {
            tx,
            last_ping_at: Instant::now(),
            last_seen_at: Instant::now(),
            connected_at: Instant::now(),
        }
    }

    pub fn try_send(&self, message: Message) {
        self.tx.try_send(message).ok();
    }
}
