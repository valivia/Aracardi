use serde::Serialize;

use crate::structs::game::state::ActiveCard;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryActiveCard {
    pub card_id: String,
    pub duration_ms: i64,
    pub expected_turns: i32,
    pub actual_turns: i32,
}

impl<'a> From<&'a ActiveCard> for TelemetryActiveCard {
    fn from(other: &'a ActiveCard) -> Self {
        TelemetryActiveCard {
            card_id: other.inner.id.clone(),
            duration_ms: other.inner.get_duration().num_milliseconds(),
            expected_turns: other.turns.original_turn_count,
            actual_turns: other.turns.turns_passed,
        }
    }
}
