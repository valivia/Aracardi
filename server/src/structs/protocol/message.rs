use std::str::FromStr;

use axum::extract::ws::{CloseFrame, Message};
use thiserror::Error;
use uuid::Uuid;

use crate::structs::{
    game::state::ClientId,
    protocol::game_update::{GameUpdate, HostUpdate},
};

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

#[derive(Debug)]
pub enum ConnectionClose {
    NotFound,
    GameFull,
    GameEnded,
    TimedOut,
    ServerError,
}

impl ToString for ConnectionClose {
    fn to_string(&self) -> String {
        match self {
            Self::GameFull => "game_full",
            Self::NotFound => "not_found",
            Self::GameEnded => "game_ended",
            Self::TimedOut => "timed_out",
            Self::ServerError => "server_error",
        }
        .into()
    }
}

impl ConnectionClose {
    pub fn to_message(&self) -> Message {
        let code = match self {
            Self::GameEnded => 1000,
            _ => 4000,
        };

        Message::Close(Some(CloseFrame {
            code,
            reason: self.to_string().into(),
        }))
    }
}

pub enum OutgoingMessage {
    ClientId(ClientId),
    Update(GameUpdate),
}

impl OutgoingMessage {
    pub fn get_key(&self) -> &str {
        match self {
            OutgoingMessage::ClientId(_) => "client_id",
            OutgoingMessage::Update(_) => "update",
        }
    }

    pub fn to_message(self) -> Message {
        let mut payload = match &self {
            OutgoingMessage::ClientId(payload) => Ok(payload.to_string()),
            OutgoingMessage::Update(payload) => serde_json::to_string(&payload),
        }
        .unwrap();

        let message_key = self.get_key();
        payload = format!("{message_key}\n{payload}");

        return Message::Text(payload.into());
    }
}

pub enum IncomingMessage {
    Connect(Option<ClientId>),
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
            "connect" => IncomingMessage::Connect(Uuid::from_str(payload).ok()),
            other => return Err(ParseError::UnknownTopic(other.to_string())),
        };

        Ok(message)
    }
}
