use std::fmt;

use axum::extract::ws::{CloseFrame, Message};

#[derive(Clone, Debug, PartialEq)]
pub enum CloseReason {
    // 1xxx — standard WebSocket codes
    GameEnded,
    ServerError,
    ServerRestart,
    // 40xx — connection / session
    NotFound,
    RateLimited,
    // 41xx — game state
    GameFull,
    // 42xx — protocol
    InvalidHandshake,
    VersionMismatch,
    // 43xx — timeout / network
    TimedOut,
}

impl CloseReason {
    pub fn code(&self) -> u16 {
        match self {
            Self::GameEnded => 1000,
            Self::ServerError => 1011,
            Self::ServerRestart => 1012,

            Self::NotFound => 4000,
            Self::RateLimited => 4001,

            Self::GameFull => 4100,

            Self::InvalidHandshake => 4200,
            Self::VersionMismatch => 4201,

            Self::TimedOut => 4300,
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::GameEnded => "GAME_ENDED",
            Self::ServerError => "SERVER_ERROR",
            Self::ServerRestart => "SERVER_RESTART",

            Self::NotFound => "NOT_FOUND",
            Self::RateLimited => "RATE_LIMITED",

            Self::GameFull => "GAME_FULL",

            Self::InvalidHandshake => "INVALID_HANDSHAKE",
            Self::VersionMismatch => "VERSION_MISMATCH",

            Self::TimedOut => "TIMED_OUT",
        }
    }

    pub fn to_message(&self) -> Message {
        Message::Close(Some(CloseFrame {
            code: self.code(),
            reason: self.reason().into(),
        }))
    }

    pub fn is_refusal(&self) -> bool {
        match self {
            Self::InvalidHandshake | Self::VersionMismatch => true,
            _ => false,
        }
    }
}

impl fmt::Display for CloseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisconnectReason {
    Close(CloseReason),
    StreamError,
    StreamEnded,
    ClosedByClient,
}

impl DisconnectReason {
    pub fn to_message(&self) -> Option<Message> {
        match self {
            Self::Close(reason) => Some(reason.to_message()),
            _ => None,
        }
    }

    pub fn is_intentional(&self) -> bool {
        self == &Self::ClosedByClient
    }
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Close(reason) => write!(f, "{reason}"),
            Self::StreamError => write!(f, "STREAM_ERROR"),
            Self::StreamEnded => write!(f, "STREAM_ENDED"),
            Self::ClosedByClient => write!(f, "CLOSED_BY_CLIENT"),
        }
    }
}
