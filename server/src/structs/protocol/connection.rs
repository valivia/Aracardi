use axum::extract::ws::{CloseFrame, Message};

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
