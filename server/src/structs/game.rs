use std::collections::HashMap;

use axum::extract::ws::Message;
use nanoid::nanoid;

use crate::structs::{
    game_state::GameState,
    player::{Player, PlayerId, Role, Tx},
};

pub type GameId = String;

pub struct Game {
    id: GameId,
    created_at: std::time::Instant,

    host: Option<PlayerId>,
    players: HashMap<PlayerId, Player>,

    state: GameState,
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Game {
            id,
            created_at: std::time::Instant::now(),
            host: None,
            players: HashMap::new(),
            state: GameState::default(),
        }
    }
}

impl Game {
    pub fn generate_id() -> GameId {
        // TODO
        nanoid!(6, &nanoid::alphabet::SAFE)
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn host(&self) -> Option<&PlayerId> {
        self.host.as_ref()
    }

    pub fn add_player(&mut self, tx: Tx) -> PlayerId {
        let id = Player::generate_id();
        let role = if self.host.is_none() {
            self.host = Some(id.clone());
            Role::Host
        } else {
            Role::Player
        };

        println!("Adding player {id} ({role:?}) to game {}", self.id);

        let player = Player::new(tx, role);
        self.players.insert(id.clone(), player);

        return id;
    }

    pub fn update_player(&mut self, id: &PlayerId, tx: Tx) {
        if let Some(player) = self.players.get_mut(id) {
            println!("Updated player {id} in game {}", self.id);
            player.tx = tx;
        }
    }

    pub fn remove_player(&mut self, id: &PlayerId) {
        println!("Removing player {id} from game {}", self.id);
        self.players.remove(id);
    }

    pub fn player(&self, id: &PlayerId) -> Option<&Player> {
        self.players.get(id)
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    pub fn broadcast(&self, msg: Message) {
        for player in &self.players {
            let _ = player.1.tx().try_send(msg.clone());
        }
    }
}
