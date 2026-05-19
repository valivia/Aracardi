use axum::extract::ws::Message;

use crate::structs::{game::state::ClientId, protocol::message::game_update::GameUpdate};

pub enum OutgoingMessage {
    ClientId(ClientId),
    GameUpdate(GameUpdate),
}

impl OutgoingMessage {
    pub fn get_key(&self) -> &str {
        match self {
            OutgoingMessage::ClientId(_) => "CLIENT_ID",
            OutgoingMessage::GameUpdate(_) => "GAME_UPDATE",
        }
    }

    pub fn to_message(self) -> Message {
        let mut payload = match &self {
            OutgoingMessage::ClientId(payload) => Ok(payload.to_string()),
            OutgoingMessage::GameUpdate(payload) => serde_json::to_string(&payload),
        }
        .unwrap();

        let message_key = self.get_key();
        payload = format!("{message_key}\n{payload}");

        return Message::Text(payload.into());
    }
}
