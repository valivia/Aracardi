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
    pub game_id: String,
    pub host_id: String,
}

pub struct AppState {
    pub games: DashMap<String, Game>,
    pub cards: Arc<RwLock<HashMap<String, AddonCard>>>,
    pub telemetry: Telemetry,
}

impl AppState {
    pub fn create_game(&self) -> CreatedGame {
        let mut game_id = Game::generate_join_code();

        while self.games.contains_key(&game_id) {
            game_id = Game::generate_join_code();
        }

        let game = Game::new(game_id.clone());

        let response = CreatedGame {
            game_id: game_id.clone(),
            host_id: game.host_id.clone(),
        };

        info!("[game] {game_id} | Creating new game");

        self.games.insert(game_id.clone(), game);

        return response;
    }
}
