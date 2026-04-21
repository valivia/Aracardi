use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::structs::game::state::{Card, GameState, Player};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdate {
    pub players: Option<Vec<Player>>,
    pub current_player_id: Option<String>,

    pub current_card: Option<Card>,
    pub active_cards: Option<Vec<Card>>,

    pub host_connected: Option<bool>,
}

impl GameUpdate {
    pub fn empty() -> Self {
        GameUpdate {
            current_player_id: None,
            active_cards: None,
            current_card: None,
            players: None,
            host_connected: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.current_player_id.is_none()
            && self.active_cards.is_none()
            && self.current_card.is_none()
            && self.players.is_none()
    }

    pub fn from_game(state: GameState) -> Self {
        GameUpdate {
            current_player_id: state.current_player_id,
            active_cards: Some(state.active_cards),
            current_card: state.current_card,
            players: Some(state.players.clone()),
            host_connected: None,
        }
    }
}
