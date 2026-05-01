use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    structs::{game::Game, telemetry::Telemetry},
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
            game_id: game.host_id.to_string(),
            host_id: game.host_id.to_string(),
        };

        info!("[game] {join_code} | Creating new game");

        self.games.insert(join_code.clone(), game);

        return response;
    }
}
