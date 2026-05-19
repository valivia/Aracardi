use crate::structs::{
    game::{
        Game, GameId,
        state::{ActiveCard, CurrentCard},
    },
    telemetry::{
        active_card::TelemetryActiveCard, card::TelemetryCard, game::TelemetryGame,
        player::TelemetryPlayer,
    },
};

pub enum TelemetryEventType {
    GameEnded(TelemetryGame),
    CardViewed(TelemetryCard),
    ActiveCardDismissed(TelemetryActiveCard),
}

pub struct TelemetryEvent(pub TelemetryEventType);

impl TelemetryEvent {
    pub fn from_card_viewed(card: &CurrentCard, game_id: &GameId) -> Self {
        Self(TelemetryEventType::CardViewed(TelemetryCard::from_card(
            card, game_id,
        )))
    }

    pub fn from_active_card(card: &ActiveCard, game_id: &GameId) -> Self {
        Self(TelemetryEventType::ActiveCardDismissed(
            TelemetryActiveCard::from_card(card, game_id),
        ))
    }

    pub fn from_game_ended(game: &Game) -> Result<Self, ()> {
        let info = game.info.clone().ok_or(())?;
        let game_end_reason = game.game_end_reason.clone().ok_or(())?;

        Ok(Self(TelemetryEventType::GameEnded(TelemetryGame {
            id: game.id,
            join_code: game.join_code.clone(),
            players: game
                .state
                .players
                .iter()
                .map(TelemetryPlayer::from)
                .collect(),
            clients: game.clients.values().cloned().collect(),
            info,
            stats: game.stats.clone(),
            exclusion_reasons: game.get_exclusion_reasons(),
            game_end_reason,
        })))
    }
}
