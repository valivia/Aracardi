use std::sync::Arc;

use tracing::{info, warn};

use crate::structs::{
    app_state::AppState,
    game::{
        Game,
        client::ClientId,
        state::{Card, Player},
        stats::GamePlayerStats,
    },
    protocol::{
        game_update::{GameUpdate, HostCard},
        message::{IncomingMessage, OutgoingMessage},
    },
    telemetry::event::TelemetryEvent,
};

const MAX_ACTIVE_CARDS: usize = 50;
const MAX_PLAYERS: usize = 20;

impl Game {
    pub async fn on_message(
        state: Arc<AppState>,
        game_id: &String,
        client_id: &ClientId,
        text: &str,
    ) {
        // Parse incoming message
        let payload = match IncomingMessage::parse_message(text) {
            Ok(IncomingMessage::Update(p)) => p,
            Ok(_) => return,
            Err(e) => {
                warn!("[game] {game_id} | parse error from {client_id}: {e}");
                return;
            }
        };

        // Get game
        let Some(mut game) = state.games.get_mut(game_id) else {
            warn!("[game] {game_id} | update for unknown game from {client_id}");
            return;
        };

        let original_state = game.state.clone();

        // Make sure only host can update game
        if &game.host_id != client_id {
            warn!("[game] {game_id} | non-host client {client_id} tried to send update");
            return;
        }

        let mut response = GameUpdate::empty();

        // Players
        if let Some(players) = &payload.players {
            game.parse_players(players, &mut response);
        }

        // Current Player
        if let Some(current_player_id) = &payload.current_player_id {
            game.parse_current_player(current_player_id, &mut response);
        }

        // Current Card
        if let Some(current_card) = &payload.current_card {
            game.parse_current_card(current_card, &mut response, &state)
                .await;
        }

        // Active cards
        if let Some(active_cards) = &payload.active_cards {
            game.parse_active_cards(active_cards, &mut response, &state)
                .await;
        }

        // Card change
        match (&original_state.current_card, &response.current_card) {
            (Some(previous_card), Some(_)) => {
                info!(
                    "[game] {} | card ({}) played for {:.1}s",
                    game.join_code,
                    previous_card.id,
                    previous_card.get_duration() as f64 / 1000.0
                );

                state
                    .telemetry
                    .push(TelemetryEvent::from_card_viewed(game.id, previous_card));
            }
            _ => {}
        }

        if !response.is_empty() {
            game.broadcast(
                OutgoingMessage::Update(response).to_message(),
                Some(client_id),
            );
        }
    }

    fn parse_players(&mut self, players: &[Player], response: &mut GameUpdate) {
        let old_players = self.state.players.clone();
        let new_players: Vec<Player> = players
            .iter()
            .filter(|p| p.is_valid())
            .take(MAX_PLAYERS)
            .cloned()
            .collect();

        if players.len() > new_players.len() {
            warn!(
                "[game] {} | Player update contained invalid players",
                self.join_code
            )
        }

        self.stats.players += GamePlayerStats::calculate_changes(&old_players, &new_players);
        self.stats
            .used_avatars
            .extend(new_players.iter().map(|p| p.avatar.clone()));

        self.state.players = new_players;
        response.players = Some(self.state.players.clone());
    }

    fn parse_current_player(&mut self, current_player_id: &str, response: &mut GameUpdate) -> bool {
        if self.state.players.iter().any(|p| p.id == current_player_id) {
            self.state.current_player_id = Some(current_player_id.to_string());
            response.current_player_id = self.state.current_player_id.clone();
            return true;
        } else {
            warn!(
                "[game] {} | invalid player ID: {current_player_id}",
                self.id
            );
            return false;
        }
    }

    async fn parse_current_card(
        &mut self,
        current_card: &HostCard,
        response: &mut GameUpdate,
        state: &Arc<AppState>,
    ) -> bool {
        if self
            .state
            .current_card
            .as_ref()
            .is_some_and(|c| c.id == current_card.id)
        {
            warn!(
                "[game] {} | received update with unchanged card ID",
                self.id
            );
            return false;
        }

        match Card::from_update(current_card, state.clone()).await {
            Some(card) => {
                if card.has_valid_players(&self.state.players) {
                    self.state.current_card = Some(card);
                    response.current_card = self.state.current_card.clone();
                    return true;
                } else {
                    warn!("[game] {} | current_card has invalid player", self.id);
                    return false;
                }
            }
            None => {
                warn!("[game] {} | invalid card ID: {}", self.id, current_card.id);
                return false;
            }
        }
    }

    async fn parse_active_cards(
        &mut self,
        received_active_cards: &[HostCard],
        response: &mut GameUpdate,
        state: &Arc<AppState>,
    ) {
        let mut active_cards: Vec<Card> = vec![];
        for active_card in received_active_cards.iter().take(MAX_ACTIVE_CARDS) {
            match Card::from_update(active_card, state.clone()).await {
                Some(card) => {
                    if card.has_valid_players(&self.state.players) {
                        active_cards.push(card);
                    }
                }
                None => warn!(
                    "[game] {} | invalid active card ID: {}",
                    self.id, active_card.id
                ),
            }
        }
        self.state.active_cards = active_cards;
        response.active_cards = Some(self.state.active_cards.clone());
    }
}
