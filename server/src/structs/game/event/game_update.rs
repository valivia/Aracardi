use std::sync::Arc;
use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{
        Game, MAX_ACTIVE_CARD_COUNT,
        client::ClientId,
        state::{
            MAX_PLAYER_COUNT, Player,
            card::{ActiveCard, Card, CurrentCard},
        },
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
        join_code: &String,
        client_id: &ClientId,
        payload: HostUpdate,
    ) {
        // Get game
        let Some(mut game) = state.games.get_mut(join_code) else {
            warn!(
                client = %client_id,
                "[game] {join_code} | Update for unknown game"
            );
            return;
        };

        // Make sure only host can update game
        if &game.host_id != client_id {
            warn!(
                client = %client_id,
                "[game] {join_code} | Non-host client tried to send game update"
            );
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
            game.initialize_game(game_info.to_owned());
        }

        if !response.is_empty() {
            game.broadcast(
                OutgoingMessage::GameUpdate(response).to_message(),
                Some(client_id),
            );
        }
    }

    fn parse_players(&mut self, players: &[Player], response: &mut GameUpdate) {
        let new_players: Vec<Player> = players
            .iter()
            .filter(|p| {
                let is_valid = p.is_valid();
                if !is_valid {
                    warn!("[game] {} | Invalid player\n{:?}", self.join_code, &p);
                }
                is_valid
            })
            .take(MAX_PLAYER_COUNT)
            .cloned()
            .collect();

        if players.len() > new_players.len() {
            warn!(
                "[game] {} | Player update contained invalid players",
                self.join_code
            )
        }

        Self::update_players(self, new_players);
        response.players = Some(self.state.players.clone());
    }

    fn parse_current_player(&mut self, current_player_id: &str, response: &mut GameUpdate) {
        if self.state.players.iter().any(|p| p.id == current_player_id) {
            self.state.current_player_id = Some(current_player_id.to_string());
            response.current_player_id = self.state.current_player_id.clone();
        } else {
            warn!(
                "[game] {} | Invalid current player ({current_player_id})",
                self.join_code
            );
        }
    }

    async fn parse_current_card(
        &mut self,
        current_card: &HostCard,
        response: &mut GameUpdate,
        state: &Arc<AppState>,
    ) {
        match CurrentCard::from_update(&state, &self.state, current_card).await {
            Ok(card) => {
                self.update_current_card(card);
                response.current_card = self.state.current_card.clone();
            }
            Err(err) => {
                warn!(
                    "[game] {} | Invalid card ({:?})\n{:?}",
                    self.join_code, err, current_card
                );
            }
        }
    }

    async fn parse_active_cards(
        &mut self,
        received_active_cards: &[HostCard],
        response: &mut GameUpdate,
        state: &Arc<AppState>,
    ) {
        let mut new_active_cards: Vec<ActiveCard> = vec![];
        for active_card in received_active_cards.iter().take(MAX_ACTIVE_CARD_COUNT) {
            match ActiveCard::from_update(&state, &self.state, active_card).await {
                Ok(card) => {
                    new_active_cards.push(card);
                }
                Err(err) => warn!(
                    "[game] {} | Invalid active card ({:?})\n{:?}",
                    self.join_code, err, active_card
                ),
            }
        }

        self.update_active_cards(new_active_cards);
        response.active_cards = Some(self.state.active_cards.clone());
    }
}
