use std::str::FromStr;
use uuid::Uuid;

use crate::structs::{
    game::{client::preferences::ClientPreferences, state::ClientId},
    protocol::message::{ParseError, host_update::HostUpdate},
};

pub enum IncomingMessage {
    Check,
    Connect(Option<ClientId>),
    GameUpdate(HostUpdate),
    ClientUpdate(ClientPreferences),
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
            "CHECK" => IncomingMessage::Check,
            "CONNECT" => IncomingMessage::Connect(Uuid::from_str(payload).ok()),
            "CLIENT_UPDATE" => IncomingMessage::ClientUpdate(parse_json!(ClientPreferences)?),
            "GAME_UPDATE" => IncomingMessage::GameUpdate(parse_json!(HostUpdate)?),
            other => return Err(ParseError::UnknownTopic(other.to_string())),
        };

        Ok(message)
    }
}
