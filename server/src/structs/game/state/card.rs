use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::structs::{
    app_state::AppState, game::state::Player, protocol::message::host_update::HostCard,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardTurns {
    #[serde(skip)]
    pub original_turn_count: i32,
    #[serde(skip_serializing)]
    pub turns_passed: i32,

    pub turns_left: i32,
}

impl Default for CardTurns {
    fn default() -> Self {
        Self {
            original_turn_count: 0,
            turns_passed: 0,
            turns_left: 0,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub title: Option<String>,
    pub text: String,
    pub image: bool,
    pub players: Vec<String>,
    #[serde(flatten)]
    pub turns: Option<CardTurns>,
    pub time_limit: Option<u32>,
    pub instance_id: String,

    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
}

impl Card {
    pub async fn from_update(card: &HostCard, state: Arc<AppState>) -> Option<Self> {
        let cards = state.cards.read().await;
        let addon_card = cards.get(&card.id)?;

        let turns: Option<CardTurns>;
        if let (Some(card_turns), Some(addon_turns)) = (&card.turns, addon_card.turns) {
            turns = Some(CardTurns {
                original_turn_count: addon_turns,
                turns_left: card_turns.turns_left,
                turns_passed: card_turns.turns_passed,
            });
        } else {
            turns = None;
        }

        Some(Card {
            id: addon_card.id.clone(),
            instance_id: card.instance_id.clone(),
            title: addon_card.title.clone(),
            text: addon_card.text.clone(),
            image: addon_card.image,
            players: card.players.clone(),
            turns,
            time_limit: addon_card.time_limit,

            created_at: Utc::now(),
        })
    }

    pub fn has_valid_players(&self, game_players: &Vec<Player>) -> bool {
        for card_player in &self.players {
            if !game_players.iter().any(|p| &p.name == card_player) {
                return false;
            }
        }
        return true;
    }

    pub fn get_duration(&self) -> i64 {
        (Utc::now() - self.created_at).num_milliseconds()
    }
}
