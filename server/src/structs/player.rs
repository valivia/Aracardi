use axum::extract::ws::Message;
use nanoid::nanoid;
use tokio::sync::mpsc;

pub type Tx = mpsc::Sender<Message>;

pub const PLAYER_ID_LENGTH: usize = 10;
pub type PlayerId = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    Host,
    Player,
}

pub struct Player {
    pub tx: Tx,
    pub role: Role,
    pub last_seen: std::time::Instant,
}

impl Player {
    pub fn new(tx: Tx, role: Role) -> Self {
        Player {
            tx,
            role,
            last_seen: std::time::Instant::now(),
        }
    }

    pub fn generate_id() -> PlayerId {
        nanoid!(PLAYER_ID_LENGTH, &nanoid::alphabet::SAFE)
    }

    pub fn tx(&self) -> &Tx {
        &self.tx
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn last_seen(&self) -> std::time::Instant {
        self.last_seen
    }
}
