use axum::{Json, extract::State};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;
use tracing::{debug, info, warn};

use crate::{
    ACTIVE_GAME_COUNTER, AppState,
    structs::{
        app_state::CreatedGame,
        game::{GameEndReason, MAX_GAME_DURATION, MAX_SETUP_DURATION},
    },
};

const CHECK_INTERVAL: Duration = Duration::from_secs(60);

pub async fn handler(State(state): State<Arc<AppState>>) -> Json<CreatedGame> {
    let game_info = state.create_game(&state);

    // Check if game has been intialized within the setup_timeout
    let join_code = game_info.join_code.clone();

    tokio::spawn(async move {
        tokio::time::sleep(MAX_SETUP_DURATION).await;

        // Check if game is initialized
        if let Some(_deleted) = state
            .games
            .remove_if(&join_code, |_, game| !game.is_initialized())
        {
            ACTIVE_GAME_COUNTER.dec();
            warn!("[game] {join_code} | deleted game (NOT_INITIALIZED)");
            return;
        }

        // Check if game has been active/idle too long
        let deadline = Instant::now() + MAX_GAME_DURATION;
        loop {
            let mut already_deleted = true;

            if state
                .games
                .remove_if_mut(&join_code, |_, game| {
                    already_deleted = false;
                    if Instant::now() > deadline {
                        warn!(
                            "[game] {join_code} |🗑️  Deleted game ({})",
                            GameEndReason::MaxDurationReached
                        );
                        game.close(GameEndReason::MaxDurationReached);
                        return true;
                    }

                    if game.is_idle() {
                        info!(
                            "[game] {join_code} |🗑️  Deleted game ({})",
                            GameEndReason::Idle
                        );
                        game.close(GameEndReason::Idle);
                        return true;
                    }

                    return false;
                })
                .is_some()
            {
                ACTIVE_GAME_COUNTER.dec();
                break;
            }

            // game deleted gracefully
            if already_deleted {
                break;
            }

            tokio::time::sleep(CHECK_INTERVAL).await;
        }

        debug!("[game] {} | Lifecycle thread closed", join_code)
    });

    Json(game_info)
}
