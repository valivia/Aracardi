use axum::extract::ws::Message;
use thiserror::Error;

use crate::structs::protocol::game_update::{GameUpdate, HostUpdate};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing newline separator")]
    MissingNewline,
    #[error("unknown topic: {0}")]
    UnknownTopic(String),
    #[error("invalid JSON for topic '{topic}': {source}")]
    InvalidJson {
        topic: String,
        source: serde_json::Error,
    },
}

pub enum OutgoingMessage {
    PlayerId(String),
    Update(GameUpdate),
}

impl OutgoingMessage {
    pub fn get_key(&self) -> &str {
        match self {
            OutgoingMessage::PlayerId(_) => "player_id",
            OutgoingMessage::Update(_) => "update",
        }
    }

    pub fn to_message(self) -> Message {
        let mut payload = match &self {
            OutgoingMessage::Update(payload) => serde_json::to_string(&payload),
            OutgoingMessage::PlayerId(payload) => Ok(payload.clone()),
        }
        .unwrap();

        let message_key = self.get_key();
        payload = format!("{message_key}\n{payload}");

        return Message::Text(payload.into());
    }
}

pub enum IncomingMessage {
    Connect(String),
    Update(HostUpdate),
}

impl IncomingMessage {
    pub fn parse_message(raw: &str) -> Result<IncomingMessage, ParseError> {
        let (topic, payload) = raw.split_once('\n').ok_or(ParseError::MissingNewline)?;

        macro_rules! parse_json {
            ($type:ty) => {
                serde_json::from_str::<$type>(payload).map_err(|source| ParseError::InvalidJson {
                    topic: topic.to_string(),
                    source,
                })
            };
        }

        let message = match topic {
            "update" => IncomingMessage::Update(parse_json!(HostUpdate)?),
            "connect" => IncomingMessage::Connect(payload.to_string()),
            other => return Err(ParseError::UnknownTopic(other.to_string())),
        };

        Ok(message)
    }
}
