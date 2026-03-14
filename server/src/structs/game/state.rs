use serde::{Deserialize, Serialize};

use crate::structs::game::client::ClientId;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub title: Option<String>,
    pub text: String,
    pub image: bool,
    pub players: Vec<String>,
    pub turns: Option<i32>,
    pub time_limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub avatar: String,

    #[serde(skip_serializing)]
    pub is_hand_picked: bool,
}

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
