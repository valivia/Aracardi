use crate::structs::{
    game::{
        client::{Client, ClientId},
        state::GameState,
    },
    protocol::{game_update::GameUpdate, message::OutgoingMessage},
};
use nanoid::nanoid;
use std::collections::HashMap;

pub mod client;
pub mod message;
pub mod state;

pub type GameId = String;

#[derive(Clone)]
pub struct Game {
    id: GameId,
    created_at: std::time::Instant,

    pub host_id: ClientId,
    pub clients: HashMap<ClientId, Client>,

    state: GameState,
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Game {
            id,
            created_at: std::time::Instant::now(),
            host_id: Client::generate_id(),
            clients: HashMap::new(),
            state: GameState::default(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.state.current_card.is_some() && self.state.current_player_id.is_some()
    }

    // Game
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    pub fn is_host_connected(&self) -> bool {
        self.clients.contains_key(&self.host_id)
    }

    pub fn send_host_status(&self) {
        let mut game_update = GameUpdate::empty();
        game_update.host_connected = Some(self.is_host_connected());
        self.broadcast(OutgoingMessage::Update(game_update).to_message(), None)
    }

    // Other
    pub fn generate_id() -> GameId {
        // TODO
        nanoid!(6, &nanoid::alphabet::SAFE)
    }
}
