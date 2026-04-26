use serde::Serialize;

use crate::structs::game::state::Card;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryActiveCard {
    pub card_id: String,
    pub duration_ms: i64,
    pub expected_turns: i32,
    pub actual_turns: i32,
}

impl<'a> From<&'a Card> for TelemetryActiveCard {
    fn from(other: &'a Card) -> Self {
        let turns = other.turns.clone().unwrap_or_default();
        TelemetryActiveCard {
            card_id: other.id.clone(),
            duration_ms: other.get_duration(),
            expected_turns: turns.original_turn_count,
            actual_turns: turns.turns_left,
        }
    }
}
