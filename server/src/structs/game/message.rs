use std::sync::Arc;

use axum::extract::ws::Message;
use tracing::{debug, warn};

use crate::structs::{
    app_state::AppState,
    game::{Game, GameId, client::ClientId, state::Card},
    protocol::{
        game_update::GameUpdate,
        topic::{IncomingMessage, OutgoingMessage},
    },
};

impl Game {
    pub async fn on_message(
        state: Arc<AppState>,
        game_id: &GameId,
        client_id: &ClientId,
        msg: Message,
    ) {
        match &msg {
            Message::Close(_) => {
                debug!("Player: {client_id} disconnect, game: {game_id}");
                return;
            }
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {
                debug!("Player: {client_id} pong, game: {game_id}");
                if let Some(mut game) = state.games.get_mut(game_id) {
                    if let Some(client) = game.clients.get_mut(client_id) {
                        client.last_seen = std::time::Instant::now();
                    }
                }
                return;
            }
            Message::Text(text) => match IncomingMessage::parse_message(text) {
                Ok(IncomingMessage::Update(payload)) => {
                    if let Some(mut game) = state.games.get_mut(game_id) {
                        let mut response = GameUpdate::empty();

                        // Active cards
                        if let Some(active_cards) = &payload.active_cards {
                            game.state.active_cards = active_cards.clone();
                            response.active_cards = Some(game.state.active_cards.clone());
                        }

                        // Current card
                        if let Some(current_card) = &payload.current_card {
                            let cards = state.cards.read().await;
                            // let mut players = Vec::new();

                            // for card_player_id in &current_card.players {
                            //     if let Some(player) =
                            //         game.state.players.get(card_player_id)
                            //     {
                            //         players.push(player.name.clone());
                            //     } else {
                            //         players.push("????".to_string());
                            //         warn!(
                            //             "Invalid player ID in card: {}",
                            //             card_player_id
                            //         );
                            //     }
                            // }

                            if let Some(addon_card) = cards.get(&current_card.id) {
                                let card = Card {
                                    id: addon_card.id.clone(),
                                    title: addon_card.title.clone(),
                                    text: addon_card.text.clone(),
                                    image: addon_card.image,
                                    players: current_card.players.clone(),
                                    turns: addon_card.turns,
                                    time_limit: addon_card.time_limit,
                                };
                                game.state.current_card = Some(card.clone());
                                response.current_card = game.state.current_card.clone();
                            } else {
                                warn!("Invalid card ID: {}", current_card.id);
                            }
                        }

                        // Players
                        if let Some(players) = &payload.players {
                            game.state.players = players.clone();
                            response.players = Some(game.state.players.clone());
                        }

                        // Current player
                        if let Some(current_player_id) = &payload.current_player_id {
                            if let Some(current_player) = game
                                .state
                                .players
                                .iter()
                                .find(|p| p.id == *current_player_id)
                            {
                                game.state.current_player_id = Some(current_player.id.clone());
                                response.current_player_id = game.state.current_player_id.clone();
                            } else {
                                warn!("Invalid player ID: {}", current_player_id);
                                return;
                            }
                        }

                        // Emit update to all players if there is any change
                        if !response.is_empty() {
                            game.broadcast(OutgoingMessage::Update(response).to_message());
                        }
                    }
                }
                Err(error) => warn!("Parse error: {error}"),
                _ => {}
            },
        }
    }

    pub fn broadcast(&self, msg: Message) {
        for player in &self.clients {
            let _ = player.1.tx.try_send(msg.clone()).ok();
        }
    }
}
