use serde::Serialize;

use crate::structs::{
    game::{GameId, client::Client, info::GameInfo, stats::GameStats},
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

    pub game_end_reason: Option<String>,
}
