use serde::Serialize;

use crate::structs::game::state::Card;

#[derive(Debug, Clone, Serialize)]
pub struct TelemetryCard {
    pub card_id: String,
    pub duration_ms: i64,
}

impl<'a> From<&'a Card> for TelemetryCard {
    fn from(other: &'a Card) -> Self {
        TelemetryCard {
            card_id: other.id.clone(),
            duration_ms: other.get_duration(),
        }
    }
}
