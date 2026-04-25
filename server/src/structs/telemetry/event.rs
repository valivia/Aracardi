use chrono::{DateTime, Utc};

use crate::structs::{
    game::{Game, GameId, state::Card},
    telemetry::{card::TelemetryCard, game::TelemetryGame, player::TelemetryPlayer},
};

pub enum TelemetryEventType {
    GameEnded(TelemetryGame),
    CardViewed(TelemetryCard),
    ActiveCardDismissed(),
    SetupFailed(),
}

pub struct TelemetryEvent(pub TelemetryEventType);

impl TelemetryEvent {
    pub fn from_card_viewed(card: &Card) -> Self {
        Self(TelemetryEventType::CardViewed(TelemetryCard::from(card)))
    }

    pub fn from_game_ended(game: &Game) -> Self {
        Self(TelemetryEventType::GameEnded(TelemetryGame {
            id: game.id,
            players: game
                .state
                .players
                .iter()
                .map(|player| TelemetryPlayer::from(player))
                .collect(),
            clients: game.clients.values().cloned().collect::<Vec<_>>(),
            info: game.info.clone(),
            stats: game.stats.clone(),
        }))
    }
}
