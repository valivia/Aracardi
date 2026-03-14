use crate::structs::game::{
    client::{Client, ClientId},
    state::GameState,
};
use nanoid::nanoid;
use std::collections::HashMap;

pub mod client;
pub mod message;
pub mod state;

pub type GameId = String;

pub struct Game {
    id: GameId,
    created_at: std::time::Instant,

    pub host_id: Option<ClientId>,
    pub clients: HashMap<ClientId, Client>,

    state: GameState,
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Game {
            id,
            created_at: std::time::Instant::now(),
            host_id: None,
            clients: HashMap::new(),
            state: GameState::default(),
        }
    }

    // Game
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    // Other
    pub fn generate_id() -> GameId {
        // TODO
        nanoid!(6, &nanoid::alphabet::SAFE)
    }
}
