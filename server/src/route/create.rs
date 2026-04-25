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
        if let Some(_deleted) = state
            .games
            .remove_if(&game_info_clone.game_id, |_, game| !game.is_initialized())
        {
            warn!("[game] {} | Was not initialized", game_info_clone.game_id);
            return;
        }

        // Check if game is dead at an interval
        loop {
            tokio::time::sleep(Duration::from_mins(1)).await;

            if let Some(_deleted) = state
                .games
                .remove_if(&game_info_clone.game_id, |_, game| !game.is_idle())
            {
                warn!("[game] {} | Was idle for too long", game_info_clone.game_id);
                return;
            }
        }
    });

    Json(game_info)
}
