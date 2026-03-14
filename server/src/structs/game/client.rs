use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use nanoid::nanoid;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::structs::{
    app_state::AppState,
    game::Game,
    protocol::{
        game_update::GameUpdate,
        topic::{IncomingMessage, OutgoingMessage},
    },
};

pub type Tx = mpsc::Sender<Message>;

pub const CLIENT_ID_LENGTH: usize = 10;
pub type ClientId = String;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Host,
    Player,
}

pub struct Client {
    pub tx: Tx,
    pub role: Role,
    pub last_seen: std::time::Instant,
}

impl Client {
    pub fn new(tx: Tx, role: Role) -> Self {
        Client {
            tx,
            role,
            last_seen: std::time::Instant::now(),
        }
    }

    pub fn generate_id() -> ClientId {
        nanoid!(CLIENT_ID_LENGTH, &nanoid::alphabet::SAFE)
    }

    pub fn send(&self, message: Message) {
        self.tx.try_send(message).ok();
    }

    pub async fn authenticate(
        socket: &mut WebSocket,
        state: &Arc<AppState>,
        tx: Tx,
        game_id: String,
    ) -> Result<String, ()> {
        let player_id = loop {
            let msg = match socket.recv().await {
                Some(Ok(msg)) => msg,
                _ => return Err(()), // client disconnected
            };

            if let Message::Text(text) = &msg {
                match IncomingMessage::parse_message(text) {
                    Ok(IncomingMessage::Connect(requested_player_id)) => {
                        let mut game = match state.games.get_mut(&game_id) {
                            Some(g) => g,
                            None => {
                                let _ = socket.close().await;
                                return Err(());
                            }
                        };

                        // Reconnect if valid + exists
                        if requested_player_id.len() == CLIENT_ID_LENGTH
                            && game.clients.get(&requested_player_id).is_some()
                        {
                            game.update_client(&requested_player_id, tx.clone());
                            break requested_player_id.to_string();
                        }

                        // Otherwise create new player
                        let new_id = game.add_client(tx.clone());

                        break new_id;
                    }
                    Err(error) => warn!("Parse error: {error}"),
                    _ => warn!("Unexpected message type during connect"),
                }
            }
        };

        socket
            .send(OutgoingMessage::PlayerId(player_id.clone()).to_message())
            .await
            .unwrap();

        return Ok(player_id);
    }
}

impl Game {
    pub fn add_client(&mut self, tx: Tx) -> ClientId {
        let id = Client::generate_id();
        let role = if self.host_id.is_none() {
            self.host_id = Some(id.clone());
            info!(
                "Host connected to {} after {}ms",
                self.id,
                self.created_at.elapsed().as_millis()
            );
            Role::Host
        } else {
            Role::Player
        };

        debug!("Adding player {id} ({role:?}) to game {}", self.id);

        let player = Client::new(tx, role);
        self.clients.insert(id.clone(), player);

        self.sync_client(&id);

        return id;
    }

    pub fn remove_client(&mut self, id: &ClientId) {
        let is_host = self
            .clients
            .get(id)
            .map(|c| c.role == Role::Host)
            .unwrap_or(false);

        if is_host {
            self.host_id = None;
            debug!("Host disconnected from {}", self.id);
        } else {
            debug!("Removing client {id} from game {}", self.id);
            self.clients.remove(id);
        }
    }

    pub fn update_client(&mut self, id: &ClientId, tx: Tx) {
        if let Some(player) = self.clients.get_mut(id) {
            debug!("Updating client {id} in game {}", self.id);
            player.tx = tx;
        }
        self.sync_client(&id);
    }

    pub fn sync_client(&self, id: &ClientId) {
        let client = self.clients.get(id);
        if let Some(client) = client {
            client.send(
                OutgoingMessage::Update(GameUpdate::from_game_state(self.state.clone()))
                    .to_message(),
            );
        }
    }
}
