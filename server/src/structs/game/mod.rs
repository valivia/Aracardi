use crate::structs::{
    game::{
        client::{Client, ClientId},
        info::GameInfo,
        state::GameState,
        stats::GameStats,
    },
    protocol::{game_update::GameUpdate, message::OutgoingMessage},
    telemetry::{Telemetry, event::TelemetryEvent},
};
use axum::extract::ws::Message;
use nanoid::nanoid;
use std::collections::HashMap;
use uuid::Uuid;

pub mod client;
pub mod info;
pub mod message;
pub mod state;
pub mod stats;

#[derive(Clone)]
pub struct Game {
    pub id: Uuid,
    pub join_code: String,
    pub created_at: std::time::Instant,

    pub host_id: ClientId,
    pub clients: HashMap<ClientId, Client>,

    pub info: GameInfo,
    pub state: GameState,
    pub stats: GameStats,
}

impl Game {
    pub fn new(join_code: String) -> Self {
        Game {
            id: Uuid::now_v7(),
            join_code,
            created_at: std::time::Instant::now(),

            host_id: Client::generate_id(),
            clients: HashMap::new(),

            info: GameInfo::default(),
            state: GameState::default(),
            stats: GameStats::default(),
        }
    }

    // Game
    pub fn is_initialized(&self) -> bool {
        self.state.current_card.is_some() && self.state.current_player_id.is_some()
    }

    pub fn is_host_connected(&self) -> bool {
        self.clients.contains_key(&self.host_id)
    }

    // Clients
    pub fn upsert_client(&mut self, client: Client, id: Option<ClientId>) -> ClientId {
        let id = match id {
            Some(id) => {
                if id != self.host_id {
                    Client::generate_id()
                } else {
                    id
                }
            }
            None => Client::generate_id(),
        };

        self.clients.insert(id.clone(), client);

        self.sync_client(&id);

        return id;
    }

    pub fn remove_client(&mut self, client_id: &ClientId) {
        self.clients.remove(client_id);
    }

    pub fn sync_client(&self, id: &ClientId) {
        let client = self.clients.get(id);
        let mut game_update = GameUpdate::from_game(self.state.clone());
        game_update.host_connected = Some(self.is_host_connected());
        if let Some(client) = client {
            client.send(OutgoingMessage::Update(game_update).to_message());
        }
    }

    // Communication
    pub fn broadcast(&self, msg: Message, source_client: Option<&String>) {
        for (id, client) in &self.clients {
            if Some(id) == source_client {
                continue;
            }
            let _ = client.tx.try_send(msg.clone()).ok();
        }
    }

    pub fn send_host_status(&self) {
        let mut game_update = GameUpdate::empty();
        game_update.host_connected = Some(self.is_host_connected());
        self.broadcast(OutgoingMessage::Update(game_update).to_message(), None)
    }

    // Other
    pub fn generate_join_code() -> String {
        const ALPHABET: [char; 22] = [
            'A', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'V',
            'W', 'X', 'Y', '3', '4',
        ];
        nanoid!(6, &ALPHABET)
    }

    // Telemetry
    pub fn log_end(&self, telemetry: &Telemetry) {
        telemetry.push(TelemetryEvent::from_game_ended(self));
    }
}
