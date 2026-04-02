use serde::Serialize;
use uuid::Uuid;

use crate::structs::{
    game::{info::GameInfo, stats::GameStats},
    telemetry::player::TelemetryPlayer,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryGame {
    pub id: Uuid,
    pub players: Vec<TelemetryPlayer>,
    pub info: Option<GameInfo>,
    pub stats: GameStats,
}
