use crate::structs::{
    game::{
        client::{Client, ClientId},
        info::GameInfo,
        state::GameState,
        stats::GameStats,
    },
    protocol::{
        game_update::GameUpdate,
        message::{ConnectionClose, OutgoingMessage},
    },
    telemetry::{Telemetry, event::TelemetryEvent},
};
use axum::extract::ws::Message;
use chrono::Utc;
use nanoid::nanoid;
use std::{collections::HashMap, ops::Not};
use tracing::{debug, info};
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

    pub info: Option<GameInfo>,
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

            info: None,
            state: GameState::default(),
            stats: GameStats::default(),
        }
    }

    // Game
    pub fn is_initialized(&self) -> bool {
        self.info.is_some()
            && self.state.current_card.is_some()
            && self.state.current_player_id.is_some()
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

        let is_host = id == self.host_id;

        if !is_host {
            // TODO: figure out reconnect?
            self.stats.client.clients_connected += 1;
            self.sync_client(&id);
        } else if self.is_initialized() {
            // TODO: mayde add a host_has_connected field to self?
            self.stats.client.host_reconnected += 1;
        }

        info!(
            game = self.join_code,
            client = id.to_string(),
            "{} {}",
            if is_host { "Host" } else { "Client" },
            "connected"
        );

        return id;
    }

    pub fn remove_client(&mut self, client_id: &ClientId) {
        if client_id != &self.host_id {
            self.stats.client.clients_disconnected += 1;
        }
        self.clients.remove(client_id);
    }

    pub fn sync_client(&self, id: &ClientId) {
        let client = self.clients.get(id);
        let mut game_update = GameUpdate::from_game(self.state.clone());
        game_update.host_connected = Some(self.is_host_connected());
        if let Some(client) = client {
            debug!("[game] {} | {} syncing player", self.join_code, id);
            client.send(OutgoingMessage::Update(game_update).to_message());
        }
    }

    // Communication
    pub fn broadcast(&self, msg: Message, source_client: Option<&ClientId>) {
        for (id, client) in &self.clients {
            if Some(id) == source_client {
                continue;
            }
            client.send(msg.clone());
        }
    }

    pub fn send_host_status(&self) {
        let mut game_update = GameUpdate::empty();
        game_update.host_connected = Some(self.is_host_connected());
        self.broadcast(
            OutgoingMessage::Update(game_update).to_message(),
            Some(&self.host_id),
        )
    }

    // Other
    pub fn generate_join_code() -> String {
        const ALPHABET: [char; 23] = [
            'A', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'V',
            'W', 'X', 'Y', 'Z', '3', '4',
        ];
        nanoid!(6, &ALPHABET)
    }

    // Telemetry
    pub fn close(&mut self, telemetry: &Telemetry) {
        if let Some(info) = &mut self.info {
            info.ended_at_ms = Utc::now().timestamp_millis()
        }

        if self.is_initialized().not() {
            return;
        }

        // Broadcast close
        self.broadcast(ConnectionClose::GameEnded.to_message(), None);

        telemetry.push(TelemetryEvent::from_game_ended(self));
    }
}
