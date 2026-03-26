use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use nanoid::nanoid;
use tokio::sync::mpsc;
use tracing::warn;

use crate::structs::{
    app_state::AppState,
    protocol::message::{IncomingMessage, OutgoingMessage},
};

pub type Tx = mpsc::Sender<Message>;

pub const CLIENT_ID_LENGTH: usize = 10;
pub type ClientId = String;

#[derive(Clone)]
pub struct Client {
    pub tx: Tx,
    pub last_seen: std::time::Instant,
}

impl Client {
    pub fn new(tx: Tx) -> Self {
        Client {
            tx,
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

                        let client = Client::new(tx.clone());

                        let requested_player_id: Option<String> = (requested_player_id.len()
                            == CLIENT_ID_LENGTH)
                            .then_some(requested_player_id);

                        // Otherwise create new player
                        let client_id = game.upsert_client(client, requested_player_id);

                        break client_id;
                    }
                    Err(error) => warn!("Parse error: {error}"),
                    _ => warn!("Unexpected message type during connect"),
                }
            }
        };

        socket
            .send(OutgoingMessage::ClientId(player_id.clone()).to_message())
            .await
            .unwrap();

        return Ok(player_id);
    }
}
