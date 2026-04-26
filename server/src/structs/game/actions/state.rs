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

    pub fn update_active_cards(&mut self, new_active_cards: Vec<Card>) {
        // Log deleted and update others
        self.state.active_cards.iter_mut().filter_map(|card| {
            if let Some(new_card) = new_active_cards
                .iter()
                .find(|new_card| new_card.instance_id == card.instance_id)
            {
                // Updated
                info!("[card] {} | updated", card.id);
                card.turns = new_card.turns.clone();
                return Some(&card);
            } else {
                // Dismissed
                let turns = card.turns.clone().unwrap_or_default();
                info!(
                    "[card] {} | dismissed at turn {:?} out of {:?}",
                    card.id, turns.turns_passed, turns.original_turn_count
                );
                self.app_state
                    .telemetry
                    .push(TelemetryEvent::from_active_card(card));

                return None;
            }
        });

        let mut new_cards: Vec<Card> = new_active_cards
            .iter()
            .filter(|card| {
                !self
                    .state
                    .active_cards
                    .iter()
                    .any(|x| x.instance_id == card.instance_id)
            })
            .cloned()
            .collect();

        for card in &new_cards {
            info!("[card] {} | added", card.id);
        }

        // Add new
        self.state.active_cards.append(&mut new_cards);
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
