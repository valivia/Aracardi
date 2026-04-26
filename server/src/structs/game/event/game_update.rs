use std::sync::Arc;

use chrono::Utc;
use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{
        Game, MAX_ACTIVE_CARD_COUNT, MAX_PLAYER_COUNT,
        client::ClientId,
        info::GameInfo,
        state::{Card, Player},
        stats::GamePlayerStats,
    },
    protocol::message::{
        game_update::GameUpdate,
        host_update::{HostCard, HostUpdate},
        outgoing::OutgoingMessage,
    },
};

impl Game {
    pub async fn on_game_update(
        state: &Arc<AppState>,
        game_id: &String,
        client_id: &ClientId,
        payload: HostUpdate,
    ) {
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

        // Game Info
        if let Some(game_info) = &payload.info {
            game.parse_game_info(game_info.clone());
        }

        if !response.is_empty() {
            game.broadcast(
                OutgoingMessage::GameUpdate(response).to_message(),
                Some(client_id),
            );
        }
    }

    fn parse_players(&mut self, players: &[Player], response: &mut GameUpdate) {
        let old_players = self.state.players.clone();
        let new_players: Vec<Player> = players
            .iter()
            .filter(|p| p.is_valid())
            .take(MAX_PLAYER_COUNT)
            .cloned()
            .collect();

        if players.len() > new_players.len() {
            warn!(
                "[game] {} | Player update contained invalid players",
                self.join_code
            )
        }

        self.stats.player += GamePlayerStats::calculate_changes(&old_players, &new_players);
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
                self.join_code
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
        match Card::from_update(current_card, state.clone()).await {
            Some(card) => {
                if card.has_valid_players(&self.state.players) {
                    self.update_current_card(card);
                    response.current_card = self.state.current_card.clone();
                    return true;
                } else {
                    warn!(
                        "[game] {} | current_card has invalid player",
                        self.join_code
                    );
                    return false;
                }
            }
            None => {
                warn!(
                    "[game] {} | invalid card ID: {}",
                    self.join_code, current_card.id
                );
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
        let mut new_active_cards: Vec<Card> = vec![];
        for active_card in received_active_cards.iter().take(MAX_ACTIVE_CARD_COUNT) {
            match Card::from_update(active_card, state.clone()).await {
                Some(card) => {
                    // TODO: only run this check when adding the card
                    if card.has_valid_players(&self.state.players) {
                        new_active_cards.push(card);
                    }
                }
                None => warn!(
                    "[game] {} | invalid active card ID: {}",
                    self.join_code, active_card.id
                ),
            }
        }

        self.update_active_cards(new_active_cards);
        response.active_cards = Some(self.state.active_cards.clone());
    }

    fn parse_game_info(&mut self, mut game_info: GameInfo) {
        if self.info.is_some() {
            warn!(
                "[game] {} | Attempted to update game info after game start",
                self.join_code
            );
            return;
        }
        game_info.started_at_ms = Utc::now().timestamp_millis();
        // TODO: validate
        self.info = Some(game_info);
    }
}
