use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    ACTIVE_GAME_COUNTER,
    structs::{
        game::{Game, GameEndReason},
        prometheus::GAMES_CREATED_SUM,
        telemetry::Telemetry,
    },
    util::card_loader::AddonCard,
};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedGame {
    pub join_code: String,
    pub host_id: String,
    // TODO: remove this (temporary backwards compatability fix)
    pub game_id: String,
}

pub struct AppState {
    pub games: DashMap<String, Game>,
    pub cards: Arc<RwLock<HashMap<String, AddonCard>>>,
    pub telemetry: Telemetry,
}

impl AppState {
    pub fn create_game(&self, state: &Arc<AppState>) -> CreatedGame {
        let mut join_code = Game::generate_join_code();

        while self.games.contains_key(&join_code) {
            join_code = Game::generate_join_code();
        }

        let game = Game::new(join_code.clone(), state.clone());

        let response = CreatedGame {
            join_code: join_code.clone(),
            game_id: join_code.to_string(),
            host_id: game.host_id.to_string(),
        };

        info!("[game] {join_code} | 🎮 Game created");

        self.games.insert(join_code.clone(), game);

        ACTIVE_GAME_COUNTER.inc();
        GAMES_CREATED_SUM.inc();

        return response;
    }

    pub fn remove_game(&self, join_code: &String, reason: GameEndReason) {
        let Some((_id, mut game)) = self.games.remove(join_code) else {
            return;
        };

        game.close(reason.clone());

        ACTIVE_GAME_COUNTER.dec();

        info!("[game] {join_code} | 🗑️  Deleted game ({})", reason);
    }
}
