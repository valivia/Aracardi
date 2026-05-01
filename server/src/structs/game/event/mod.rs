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
        join_code: &String,
        client_id: &ClientId,
        text: &str,
    ) {
        match IncomingMessage::parse_message(text) {
            Ok(IncomingMessage::GameUpdate(payload)) => {
                Game::on_game_update(state, join_code, client_id, payload).await;
            }
            Ok(IncomingMessage::ClientUpdate(payload)) => {
                Game::on_client_update(state, join_code, client_id, payload)
            }
            Ok(_) => return,
            Err(e) => {
                warn!(
                    client = client_id.to_string(),
                    "[game] {join_code} | Parse error: {e}"
                );
                return;
            }
        };
    }
}
