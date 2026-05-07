use tokio::time::Instant;

pub use crate::structs::game::{
    client::ClientId,
    state::{
        card::{ActiveCard, CurrentCard},
        player::Player,
    },
};

pub mod card;
pub mod player;

pub const MIN_PLAYER_COUNT: usize = 2;
pub const MAX_PLAYER_COUNT: usize = 20;

#[derive(Clone, Debug)]
pub struct GameState {
    pub last_update: Instant,

    // Players
    pub players: Vec<Player>,
    pub current_player_id: Option<String>,

    // Cards
    pub active_cards: Vec<ActiveCard>,
    pub current_card: Option<CurrentCard>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            last_update: Instant::now(),

            players: vec![],
            current_player_id: None,
            current_card: None,
            active_cards: vec![],
        }
    }
}

impl GameState {
    pub fn log_update(&mut self) {
        self.last_update = Instant::now();
    }

    pub fn is_valid(&self) -> bool {
        // Invalid turn
        if self.current_card.is_none() || self.current_player_id.is_none() {
            return false;
        }

        // Invalid player count
        if !(MIN_PLAYER_COUNT..MAX_PLAYER_COUNT).contains(&self.players.len()) {
            return false;
        }

        return true;
    }
}
