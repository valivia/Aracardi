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
}
