use serde::Serialize;

use crate::structs::game::{GameId, state::CurrentCard};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryCard {
    pub game_id: GameId,
    pub card_id: String,
    pub duration_ms: i64,
}

impl TelemetryCard {
    pub fn from_card(card: &CurrentCard, game_id: &GameId) -> Self {
        Self {
            game_id: game_id.to_owned(),
            card_id: card.inner.id.clone(),
            duration_ms: card.inner.get_duration().num_milliseconds(),
        }
    }
}
