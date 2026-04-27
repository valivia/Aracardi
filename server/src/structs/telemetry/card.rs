use serde::Serialize;

use crate::structs::game::state::CurrentCard;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryCard {
    pub card_id: String,
    pub duration_ms: i64,
}

impl<'a> From<&'a CurrentCard> for TelemetryCard {
    fn from(other: &'a CurrentCard) -> Self {
        TelemetryCard {
            card_id: other.inner.id.clone(),
            duration_ms: other.inner.get_duration(),
        }
    }
}
