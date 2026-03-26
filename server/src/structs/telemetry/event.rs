use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::structs::{
    game::{Game, state::Card},
    telemetry::{card::TelemetryCard, game::TelemetryGame, player::TelemetryPlayer},
};

pub enum TelemetryEventType {
    GameEnded(TelemetryGame),
    CardViewed(TelemetryCard),
    SetupFailed(),
}

// TODO: do i keep these fields?
pub struct TelemetryEvent {
    pub game_id: Uuid,
    pub event: TelemetryEventType,
    pub ts: DateTime<Utc>,
}

impl TelemetryEvent {
    pub fn from_card_viewed(game_id: Uuid, card: &Card) -> Self {
        Self {
            game_id,
            event: TelemetryEventType::CardViewed(TelemetryCard::from(card)),
            ts: chrono::Utc::now(),
        }
    }

    pub fn from_game_ended(game: &Game) -> Self {
        Self {
            game_id: game.id,
            event: TelemetryEventType::GameEnded(TelemetryGame {
                id: game.id,
                players: game
                    .state
                    .players
                    .iter()
                    .map(|player| TelemetryPlayer::from(player))
                    .collect(),
                info: game.info.clone(),
                stats: game.stats.clone(),
            }),
            ts: chrono::Utc::now(),
        }
    }
}
