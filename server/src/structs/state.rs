use dashmap::DashMap;

use crate::structs::game::{Game, GameId};

pub struct AppState {
    pub games: DashMap<GameId, Game>,
}

impl AppState {
    pub fn create_game(&self) -> GameId {
        let mut game_id = Game::generate_id();

        while self.games.contains_key(&game_id) {
            game_id = Game::generate_id();
        }

        // game_id = "1234".to_string(); // TODO remove

        let game = Game::new(game_id.clone());

        println!("Creating new game with id {}", game_id);

        self.games.insert(game_id.clone(), game);
        game_id
    }
}
