use crate::structs::{
    game::{
        Game,
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
    pub fn from_card_viewed(card: &CurrentCard) -> Self {
        Self(TelemetryEventType::CardViewed(TelemetryCard::from(card)))
    }

    pub fn from_active_card(card: &ActiveCard) -> Self {
        Self(TelemetryEventType::ActiveCardDismissed(
            TelemetryActiveCard::from(card),
        ))
    }

    pub fn from_game_ended(game: &Game) -> Self {
        let exclusion_reasons = game.get_exclusion_reasons();

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

            should_exclude: !exclusion_reasons.is_empty(),
            exclusion_reasons,
            game_end_reason: game.game_end_reason.to_owned(),
        }))
    }
}
