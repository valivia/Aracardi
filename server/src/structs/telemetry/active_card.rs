use serde::Serialize;

use crate::structs::game::{GameId, state::ActiveCard};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryActiveCard {
    pub game_id: GameId,
    pub card_id: String,
    pub duration_ms: i64,
    pub expected_turns: i32,
    pub actual_turns: i32,
}

impl TelemetryActiveCard {
    pub fn from_card(card: &ActiveCard, game_id: &GameId) -> Self {
        Self {
            game_id: game_id.to_owned(),
            card_id: card.inner.id.clone(),
            duration_ms: card.inner.get_duration().num_milliseconds(),
            expected_turns: card.turns.original_turn_count,
            actual_turns: card.turns.turns_passed,
        }
    }
}
