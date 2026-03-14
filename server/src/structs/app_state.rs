use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    structs::game::{Game, GameId},
    util::card_loader::AddonCard,
};

pub struct AppState {
    pub games: DashMap<GameId, Game>,
    pub cards: Arc<RwLock<HashMap<String, AddonCard>>>,
}

impl AppState {
    pub fn create_game(&self) -> GameId {
        let mut game_id = Game::generate_id();

        while self.games.contains_key(&game_id) {
            game_id = Game::generate_id();
        }

        // game_id = "1234".to_string(); // TODO remove

        let game = Game::new(game_id.clone());

        info!("Creating new game with id {}", game_id);

        self.games.insert(game_id.clone(), game);
        game_id
    }
}
