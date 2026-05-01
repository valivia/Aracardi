use tracing::{info, warn};

use crate::structs::{
    game::{
        Game,
        state::{ActiveCard, CurrentCard},
    },
    telemetry::event::TelemetryEvent,
};

impl Game {
    pub fn update_current_card(&mut self, new_card: CurrentCard) {
        if self
            .state
            .current_card
            .as_ref()
            .is_some_and(|card| card == &new_card)
        {
            warn!(
                "[game] {} | Received update with unchanged card ID",
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
                "[game] {} | Card played for {:.1}s ({})",
                self.join_code,
                previous_card.inner.get_duration() as f64 / 1000.0,
                previous_card.inner.id
            );
        }

        self.state.current_card = Some(new_card);
        self.state.log_update();
    }

    pub fn update_active_cards(&mut self, new_active_cards: Vec<ActiveCard>) {
        // Log dismissed cards
        for card in &self.state.active_cards {
            if !new_active_cards
                .iter()
                .any(|c| c.inner.instance_id == card.inner.instance_id)
            {
                info!(
                    "[game] {} | Active card dismissed at turn {:?} out of {:?} ({})",
                    self.join_code,
                    card.turns.turns_passed,
                    card.turns.original_turn_count,
                    card.inner.id,
                );

                self.app_state
                    .telemetry
                    .push(TelemetryEvent::from_active_card(card));
            }
        }

        self.state.active_cards = new_active_cards
            .into_iter()
            .map(|new_card| {
                if let Some(old_card) = self
                    .state
                    .active_cards
                    .iter()
                    .find(|c| c.inner.instance_id == new_card.inner.instance_id)
                {
                    // Update card
                    ActiveCard {
                        inner: old_card.inner.clone(),
                        turns: new_card.turns,
                    }
                } else {
                    // Add card
                    new_card
                }
            })
            .collect();
    }

    pub fn flush_active_cards(&mut self) {
        for card in &self.state.active_cards {
            self.app_state
                .telemetry
                .push(TelemetryEvent::from_active_card(&card));
        }

        self.state.active_cards = Vec::new()
    }
}
