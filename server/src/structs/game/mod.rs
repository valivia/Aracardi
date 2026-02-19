use crate::structs::{
    game::state::GameState,
    player::{Player, PlayerId},
};
use nanoid::nanoid;
use std::collections::HashMap;

pub mod message;
pub mod player;
pub mod protocol;
pub mod state;

pub type GameId = String;

pub struct Game {
    id: GameId,
    created_at: std::time::Instant,

    host: Option<PlayerId>,
    players: HashMap<PlayerId, Player>,

    state: GameState,
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Game {
            id,
            created_at: std::time::Instant::now(),
            host: None,
            players: HashMap::new(),
            state: GameState::default(),
        }
    }

    // Game
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn host(&self) -> Option<&PlayerId> {
        self.host.as_ref()
    }

    // Other
    pub fn generate_id() -> GameId {
        // TODO
        nanoid!(6, &nanoid::alphabet::SAFE)
    }
}
