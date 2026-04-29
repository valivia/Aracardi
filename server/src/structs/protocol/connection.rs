use std::fmt;

use axum::extract::ws::{CloseFrame, Message};

#[derive(Debug, PartialEq)]
pub enum CloseReason {
    GameEnded,
    NotFound,
    GameFull,
    TimedOut,
    ServerError,
    ServerRestart,
}

// Spec:
// https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code
impl CloseReason {
    pub fn get(&self) -> (u16, &'static str) {
        match self {
            Self::GameEnded => (1000, "GAME_ENDED"),
            Self::NotFound => (4000, "NOT_FOUND"),
            Self::GameFull => (4100, "GAME_FULL"),
            Self::TimedOut => (3008, "TIMED_OUT"),
            Self::ServerError => (1011, "SERVER_ERROR"),
            Self::ServerRestart => (1012, "SERVER_RESTART"),
        }
    }

    pub fn to_message(&self) -> Message {
        let (code, reason) = self.get();
        Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        }))
    }
}

impl fmt::Display for CloseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get().1)
    }
}

#[derive(Debug, PartialEq)]
pub enum DisconnectReason {
    Close(CloseReason),
    StreamError,
    ClosedByClient,
}

impl DisconnectReason {
    pub fn to_message(&self) -> Option<Message> {
        match self {
            Self::Close(reason) => Some(reason.to_message()),
            _ => None,
        }
    }
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Close(reason) => write!(f, "{reason}"),
            Self::StreamError => write!(f, "STREAM_ERROR"),
            Self::ClosedByClient => write!(f, "CLOSED_BY_CLIENT"),
        }
    }
}
