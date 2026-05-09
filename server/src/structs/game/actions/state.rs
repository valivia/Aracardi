use tracing::{info, warn};

use crate::{
    structs::{
        game::{
            Game,
            state::{ActiveCard, CurrentCard, Player},
            stats::GamePlayerStats,
        },
        prometheus::CARDS_PLAYED_SUM,
        telemetry::event::TelemetryEvent,
    },
    util::human_readable::HumanReadable,
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

        self.flush_current_card();

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
                    "[game] {} | 📤 Active card dismissed at turn {:?} out of {:?} ({})",
                    self.join_code,
                    card.turns.turns_passed,
                    card.turns.original_turn_count,
                    card.inner.id,
                );

                self.app_state
                    .telemetry
                    .push(TelemetryEvent::from_active_card(card, &self.id));
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

    pub fn update_players(&mut self, new_players: Vec<Player>) {
        self.stats.player += GamePlayerStats::calculate_changes(&self.state.players, &new_players);
        self.stats
            .used_avatars
            .extend(new_players.iter().map(|p| p.avatar.clone()));

        self.state.players = new_players;
    }

    pub fn flush_current_card(&mut self) {
        if let Some(current_card) = &self.state.current_card {
            self.stats.card.register_card(&current_card);
            self.app_state
                .telemetry
                .push(TelemetryEvent::from_card_viewed(&current_card, &self.id));

            CARDS_PLAYED_SUM.inc();

            info!(
                "[game] {} | 🔷 Card played for {} ({})",
                self.join_code,
                current_card.inner.get_duration().human_readable(),
                current_card.inner.id
            );
        }
    }

    pub fn flush_active_cards(&mut self) {
        for card in &self.state.active_cards {
            self.app_state
                .telemetry
                .push(TelemetryEvent::from_active_card(&card, &self.id));
        }

        self.state.active_cards = Vec::new()
    }
}
