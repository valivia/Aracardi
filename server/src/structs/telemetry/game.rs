use serde::Serialize;

use crate::structs::{
    game::{
        GameEndReason, GameExclusionReason, GameId, client::Client, info::GameInfo,
        stats::GameStats,
    },
    telemetry::player::TelemetryPlayer,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryGame {
    #[serde(serialize_with = "uuid::serde::hyphenated::serialize")]
    pub id: GameId,
    pub join_code: String,

    pub clients: Vec<Client>,
    pub info: Option<GameInfo>,

    pub players: Vec<TelemetryPlayer>,
    pub stats: GameStats,

    pub should_exclude: bool,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub exclusion_reasons: Vec<GameExclusionReason>,
    pub game_end_reason: Option<GameEndReason>,
}
