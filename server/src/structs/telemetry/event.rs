use crate::structs::{
    game::{Game, state::Card},
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
            join_code: game.join_code.clone(),
            players: game
                .state
                .players
                .iter()
                .map(|player| TelemetryPlayer::from(player))
                .collect(),
            clients: game.clients.values().cloned().collect::<Vec<_>>(),
            info: game.info.clone(),
            stats: game.stats.clone(),

            game_end_reason: game
                .game_end_reason
                .as_ref()
                .and_then(|reason| Some(reason.get_key())),
        }))
    }
}
