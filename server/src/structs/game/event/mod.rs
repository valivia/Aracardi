use std::sync::Arc;

use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{Game, state::ClientId},
    protocol::message::incoming::IncomingMessage,
};

mod client_update;
mod game_update;

impl Game {
    pub async fn on_message(
        state: &Arc<AppState>,
        game_id: &String,
        client_id: &ClientId,
        text: &str,
    ) {
        match IncomingMessage::parse_message(text) {
            Ok(IncomingMessage::GameUpdate(payload)) => {
                Game::on_game_update(state, game_id, client_id, payload).await;
            }
            Ok(IncomingMessage::ClientUpdate(payload)) => {
                Game::on_client_update(state, game_id, client_id, payload)
            }
            Ok(_) => return,
            Err(e) => {
                warn!("[game] {game_id} | parse error from {client_id}: {e}");
                return;
            }
        };
    }
}
