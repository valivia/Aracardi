use crate::structs::{
    game::{
        client::{Client, ClientId, connection::ClientConnection, socket::ClientSocket},
        info::GameInfo,
        state::GameState,
        stats::GameStats,
    },
    protocol::{game_update::GameUpdate, message::OutgoingMessage},
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

pub type GameId = Uuid;

#[derive(Clone)]
pub struct Game {
    pub id: GameId,
    pub join_code: String,
    pub created_at: std::time::Instant,
    pub game_ended: bool,

    pub host_id: ClientId,
    pub clients: HashMap<ClientId, Client>,

    pub info: Option<GameInfo>,
    pub state: GameState,
    pub stats: GameStats,
}

impl Game {
    pub fn new(join_code: String) -> Self {
        Game {
            created_at: std::time::Instant::now(),
            game_ended: false,

            id: Uuid::now_v7(),
            join_code,

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
        self.clients
            .get(&self.host_id)
            .map(|client| client.is_connected())
            .unwrap_or(false)
    }

    // Clients
    pub fn upsert_client(
        &mut self,
        id: Option<ClientId>,
        connection: ClientConnection,
        socket: ClientSocket,
    ) -> ClientId {
        let Some(id) = id else {
            return self.new_client(connection, socket);
        };

        // Host connect
        if !self.is_host_connected() && id == self.host_id {
            let client = Client::new(id, connection, socket);
            self.clients.insert(id, client);
            info!(
                game = self.join_code,
                client = id.to_string(),
                "Host connected",
            );
            return id;
        }

        let Some(client) = self.clients.get_mut(&id) else {
            return self.new_client(connection, socket);
        };

        // TODO: Handle user trying to connect to a non-dead connection
        // TODO: Maybe also compare ip/user agent
        client.reconnect(socket);
        self.sync_client(&id);

        info!(
            game = self.join_code,
            client = id.to_string(),
            "{} reconnected",
            if id == self.host_id { "Host" } else { "Client" },
        );

        return id;
    }

    fn new_client(&mut self, connection: ClientConnection, socket: ClientSocket) -> ClientId {
        let id = Client::generate_id();
        let client = Client::new(id, connection, socket);
        self.clients.insert(id, client);
        self.sync_client(&id);

        info!(
            game = self.join_code,
            client = id.to_string(),
            "Client connected",
        );

        return id;
    }

    pub fn remove_client(&mut self, client_id: &ClientId) {
        if let Some(client) = self.clients.get_mut(&client_id) {
            if client.is_disconnected() {
                debug!(client_id = client_id.to_string(), "Attempted to double disconnect");
                return;
            }

            client.disconnect(self.game_ended);
            
            info!(
                game = self.join_code,
                client = client_id.to_string(),
                "{} disconnected",
                if client_id == &self.host_id { "Host" } else { "Client" }
            );
        }
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
        self.game_ended = true;
        if let Some(info) = &mut self.info {
            info.ended_at_ms = Utc::now().timestamp_millis()
        }

        if self.is_initialized().not() {
            return;
        }

        for (_id, client) in &mut self.clients {
            client.disconnect(self.game_ended);
        }

        telemetry.push(TelemetryEvent::from_game_ended(self));
    }
}
