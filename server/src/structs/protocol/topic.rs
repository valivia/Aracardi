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
    Update(GameUpdate),
}

impl OutgoingMessage {
    pub fn get_key(&self) -> &str {
        match self {
            OutgoingMessage::Update(_) => "update",
        }
    }

    pub fn to_message(self) -> Message {
        let mut payload = match &self {
            OutgoingMessage::Update(payload) => serde_json::to_string(&payload),
        }
        .unwrap();

        let message_key = self.get_key();
        payload = format!("{message_key}\n{payload}");

        return Message::Text(payload.into());
    }
}

pub enum IncomingMessage {
    Update(HostUpdate),
}

impl IncomingMessage {
    pub fn parse_message(raw: &str) -> Result<IncomingMessage, ParseError> {
        let (topic, json) = raw.split_once('\n').ok_or(ParseError::MissingNewline)?;

        macro_rules! parse_json {
            ($type:ty) => {
                serde_json::from_str::<$type>(json).map_err(|source| ParseError::InvalidJson {
                    topic: topic.to_string(),
                    source,
                })
            };
        }

        let message = match topic {
            "update" => IncomingMessage::Update(parse_json!(HostUpdate)?),
            other => return Err(ParseError::UnknownTopic(other.to_string())),
        };

        Ok(message)
    }
}
