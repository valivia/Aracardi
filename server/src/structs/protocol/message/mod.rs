use thiserror::Error;

pub mod game_update;
pub mod host_update;
pub mod incoming;
pub mod outgoing;

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
