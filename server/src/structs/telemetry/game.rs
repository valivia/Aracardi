use serde::Serialize;

use crate::structs::{
    game::{GameId, client::Client, info::GameInfo, stats::GameStats},
    telemetry::player::TelemetryPlayer,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryGame {
    pub id: GameId,
    pub players: Vec<TelemetryPlayer>,
    pub clients: Vec<Client>,
    pub info: Option<GameInfo>,
    pub stats: GameStats,
}
