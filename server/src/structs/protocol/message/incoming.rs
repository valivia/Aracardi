use crate::structs::{
    game::client::preferences::ClientPreferences,
    protocol::message::{ParseError, connect::Connect, host_update::HostUpdate},
};

pub enum IncomingMessage {
    Pong,
    Connect(Connect),
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
            "PONG" => IncomingMessage::Pong,
            "CONNECT" => IncomingMessage::Connect(parse_json!(Connect)?),
            "CLIENT_UPDATE" => IncomingMessage::ClientUpdate(parse_json!(ClientPreferences)?),
            "GAME_UPDATE" => IncomingMessage::GameUpdate(parse_json!(HostUpdate)?),
            other => return Err(ParseError::UnknownTopic(other.to_string())),
        };

        Ok(message)
    }
}
