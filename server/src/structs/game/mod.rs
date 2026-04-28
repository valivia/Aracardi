use crate::structs::{
    app_state::AppState,
    game::{
        client::{Client, ClientId},
        info::GameInfo,
        state::GameState,
        stats::GameStats,
    },
    protocol::message::{game_update::GameUpdate, outgoing::OutgoingMessage},
    telemetry::event::TelemetryEvent,
};
use axum::extract::ws::Message;
use chrono::Utc;
use nanoid::nanoid;
use std::{collections::HashMap, ops::Not, sync::Arc, time::Duration};
use tokio::time::Instant;
use uuid::Uuid;

pub mod actions;
pub mod client;
pub mod event;
pub mod info;
pub mod state;
pub mod stats;

pub type GameId = Uuid;

const MAX_CLIENT_COUNT: usize = 32;
const MAX_PLAYER_COUNT: usize = 20;
const MAX_ACTIVE_CARD_COUNT: usize = 50;

pub const MAX_SETUP_DURATION: Duration = Duration::from_secs(30);
pub const MAX_IDLE_DURATION: Duration = Duration::from_mins(60);
pub const MAX_GAME_DURATION: Duration = Duration::from_hours(12);
#[cfg(debug_assertions)]
pub const MAX_HOST_ABSENCE: Duration = Duration::from_secs(5);
#[cfg(not(debug_assertions))]
pub const MAX_HOST_ABSENCE: Duration = Duration::from_mins(5);

#[derive(Clone)]
pub enum GameEndReason {
    HostLeft,
    Idle,
    MaxDurationReached,
}

impl GameEndReason {
    pub fn get_key(&self) -> String {
        match self {
            Self::HostLeft => "HOST_LEFT",
            Self::Idle => "IDLE",
            Self::MaxDurationReached => "MAX_DURATION_REACHED",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct Game {
    pub id: GameId,
    pub join_code: String,
    pub created_at: Instant,
    pub game_end_reason: Option<GameEndReason>,

    pub host_id: ClientId,
    pub clients: HashMap<ClientId, Client>,

    pub info: Option<GameInfo>,
    pub state: GameState,
    pub stats: GameStats,

    pub app_state: Arc<AppState>,
}

impl Game {
    pub fn new(join_code: String, app_state: Arc<AppState>) -> Self {
        Game {
            created_at: Instant::now(),
            game_end_reason: None,

            id: Uuid::now_v7(),
            join_code,

            host_id: Client::generate_id(),
            clients: HashMap::new(),

            info: None,
            state: GameState::default(),
            stats: GameStats::default(),

            app_state,
        }
    }

    // Game
    pub fn is_initialized(&self) -> bool {
        self.info.is_some()
            && self.state.current_card.is_some()
            && self.state.current_player_id.is_some()
    }

    pub fn is_idle(&self) -> bool {
        if Instant::now().duration_since(self.state.last_update) >= MAX_IDLE_DURATION {
            return true;
        }

        false
    }

    pub fn is_host_connected(&self) -> bool {
        self.clients
            .get(&self.host_id)
            .map(|client| client.is_connected())
            .unwrap_or(false)
    }

    pub fn is_full(&self) -> bool {
        return self.clients.iter().count() >= MAX_CLIENT_COUNT;
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
            OutgoingMessage::GameUpdate(game_update).to_message(),
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
    pub fn close(&mut self, reason: GameEndReason) {
        self.game_end_reason = Some(reason);
        if let Some(info) = &mut self.info {
            info.ended_at_ms = Utc::now().timestamp_millis()
        }

        if self.is_initialized().not() {
            return;
        }

        for (_id, client) in &mut self.clients {
            client.disconnect(&self.game_end_reason.clone());
        }

        self.flush_active_cards();

        self.app_state
            .telemetry
            .push(TelemetryEvent::from_game_ended(&self));
    }
}
