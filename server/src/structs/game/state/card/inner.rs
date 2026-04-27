use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{structs::game::state::Player, util::card_loader::AddonCard};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InnerCard {
    pub id: String,
    pub title: Option<String>,
    pub text: String,
    pub image: bool,
    pub players: Vec<String>,
    pub time_limit: Option<u32>,
    pub instance_id: String,

    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
}

impl InnerCard {
    pub fn from_addon_card(
        addon_card: AddonCard,
        instance_id: String,
        players: Vec<String>,
    ) -> InnerCard {
        return InnerCard {
            id: addon_card.id,
            title: addon_card.title,
            text: addon_card.text,
            image: addon_card.image,
            players: players.clone(),
            time_limit: addon_card.time_limit,
            instance_id: instance_id.clone(),
            created_at: Utc::now(),
        };
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
