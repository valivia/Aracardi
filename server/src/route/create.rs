use axum::{Json, extract::State};
use std::{sync::Arc, time::Duration};
use tracing::warn;

use crate::{AppState, structs::app_state::CreatedGame};

const SETUP_TIMEOUT: Duration = Duration::from_mins(1);

pub async fn handler(State(state): State<Arc<AppState>>) -> Json<CreatedGame> {
    let game_info = state.create_game();

    // Check if game has been intialized within the setup_timeout
    let game_info_clone = game_info.clone();
    tokio::spawn(async move {
        tokio::time::sleep(SETUP_TIMEOUT).await;
        // TODO: make sure its not BEING initialized
        let deleted = state
            .games
            .remove_if(&game_info_clone.game_id, |_, game| !game.is_initialized());

        if deleted.is_some() {
            warn!("[game] {} | Was not initialized", game_info_clone.game_id)
        }
    });

    Json(game_info)
}
