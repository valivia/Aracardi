use tracing::{info, warn};

use crate::structs::{
    game::{Game, state::Card},
    telemetry::event::TelemetryEvent,
};

impl Game {
    pub fn update_current_card(&mut self, new_card: Card) {
        if self
            .state
            .current_card
            .as_ref()
            .is_some_and(|card| card == &new_card)
        {
            warn!(
                "[game] {} | received update with unchanged card ID",
                self.join_code
            );
            return;
        }

        if let Some(previous_card) = &self.state.current_card {
            self.stats.card.register_card(&previous_card);
            self.app_state
                .telemetry
                .push(TelemetryEvent::from_card_viewed(&previous_card));

            info!(
                "[game] {} | card ({}) played for {:.1}s",
                self.join_code,
                previous_card.id,
                previous_card.get_duration() as f64 / 1000.0
            );
        }

        self.state.current_card = Some(new_card);
        self.state.log_update();
    }
}
