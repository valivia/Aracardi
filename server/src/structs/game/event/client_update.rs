use std::sync::Arc;

use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{Game, client::preferences::ClientPreferences, state::ClientId},
};

impl Game {
    pub fn on_client_update(
        state: &Arc<AppState>,
        game_id: &String,
        client_id: &ClientId,
        payload: ClientPreferences,
    ) {
        // Get game
        let Some(mut game) = state.games.get_mut(game_id) else {
            warn!("[game] {game_id} | update for unknown game from {client_id}");
            return;
        };

        let Some(client) = game.clients.get_mut(client_id) else {
            warn!("[game] {game_id} | update for unknown client {client_id}");
            return;
        };

        client.preferences = Some(payload);
    }
}
