use axum::{Json, extract::State};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;
use tracing::{info, warn};

use crate::{
    AppState,
    structs::{
        app_state::CreatedGame,
        game::{GameEndReason, MAX_GAME_DURATION, MAX_SETUP_DURATION},
    },
};

const CHECK_INTERVAL: Duration = Duration::from_secs(60);

pub async fn handler(State(state): State<Arc<AppState>>) -> Json<CreatedGame> {
    let game_info = state.create_game(&state);

    // Check if game has been intialized within the setup_timeout
    let game_id = game_info.game_id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(MAX_SETUP_DURATION).await;

        // Check if game is initialized
        if let Some(_deleted) = state
            .games
            .remove_if(&game_id, |_, game| !game.is_initialized())
        {
            warn!("[game] {} | Was not initialized", game_id);
            return;
        }

        // Check if game has been active/idle too long
        let deadline = Instant::now() + MAX_GAME_DURATION;
        loop {
            let mut already_deleted = true;

            if state
                .games
                .remove_if_mut(&game_id, |_, game| {
                    already_deleted = false;
                    if Instant::now() > deadline {
                        warn!("[game] {} | Exceeded max lifetime", game_id);
                        game.close(GameEndReason::MaxDurationReached);
                        return true;
                    }

                    if game.is_idle() {
                        info!("[game] {} | Was idle for too long", game_id);
                        game.close(GameEndReason::Idle);
                        return true;
                    }

                    return false;
                })
                .is_some()
            {
                break;
            }

            // game deleted gracefully
            if already_deleted {
                break;
            }

            tokio::time::sleep(CHECK_INTERVAL).await;
        }
    });

    Json(game_info)
}
