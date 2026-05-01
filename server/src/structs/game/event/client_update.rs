use std::sync::Arc;

use tracing::warn;

use crate::structs::{
    app_state::AppState,
    game::{Game, client::preferences::ClientPreferences, state::ClientId},
};

impl Game {
    pub fn on_client_update(
        state: &Arc<AppState>,
        join_code: &String,
        client_id: &ClientId,
        payload: ClientPreferences,
    ) {
        // Get game
        let Some(mut game) = state.games.get_mut(join_code) else {
            warn!("[game] {join_code} | Update for unknown game from {client_id}");
            return;
        };

        let Some(client) = game.clients.get_mut(client_id) else {
            warn!("[game] {join_code} | Update for unknown client {client_id}");
            return;
        };

        client.preferences = Some(payload);
    }
}
