pub use crate::structs::game::{
    client::ClientId,
    state::{card::Card, player::Player},
};

pub mod card;
pub mod player;

#[derive(Clone, Debug)]
pub struct GameState {
    // Players
    pub players: Vec<Player>,
    pub current_player_id: Option<ClientId>,

    // Cards
    pub active_cards: Vec<Card>,
    pub current_card: Option<Card>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            players: vec![],
            current_player_id: None,
            current_card: None,
            active_cards: vec![],
        }
    }
}

impl GameState {}
