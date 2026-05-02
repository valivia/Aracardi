use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitStream, StreamExt};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::debug;

use crate::{
    AppState,
    structs::{
        game::{Game, state::ClientId},
        protocol::connection::{CloseReason, DisconnectReason},
    },
};

pub async fn receive(
    state: &Arc<AppState>,
    game_join_id: &String,
    client_id: &ClientId,
    mut receiver: SplitStream<WebSocket>,
    mut timeout_rx: oneshot::Receiver<()>,
) -> DisconnectReason {
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    None => {
                        debug!("[game] {game_join_id} | {client_id} stream ended for");
                        return DisconnectReason::StreamEnded;
                    }
                    Some(Err(e)) => {
                        debug!("[game] {game_join_id} | {client_id} received error: {e}");
                        return DisconnectReason::StreamError;
                    }
                    Some(Ok(msg)) => match msg {
                        Message::Close(_) => {
                            return DisconnectReason::ClosedByClient;
                        },
                        Message::Pong(_) => {
                            if let Some(mut game) = state.games.get_mut(game_join_id) {
                                if let Some(client) = game.clients.get_mut(&client_id) {
                                    client.pong();
                            }
                            }
                        }
                        Message::Text(text) => {
                            Game::on_message(&state, &game_join_id, &client_id, &text).await;
                        }
                        _ => {}
                    },
                }
            }

            _ = &mut timeout_rx => {
                return DisconnectReason::Close(CloseReason::TimedOut);
            }
        }
    }
}
