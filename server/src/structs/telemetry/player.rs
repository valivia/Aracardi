use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::structs::game::state::Player;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryPlayer {
    pub name: String,
    pub avatar: String,

    pub is_hand_picked: bool,
    pub was_loaded: bool,
}

impl<'a> From<&'a Player> for TelemetryPlayer {
    fn from(other: &'a Player) -> Self {
        TelemetryPlayer {
            name: other.name.clone(),
            avatar: other.avatar.clone(),
            is_hand_picked: other.is_hand_picked,
            was_loaded: other.was_loaded,
        }
    }
}
