use axum::extract::ws::{CloseFrame, Message};

// Spec:
// https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code

#[derive(Debug)]
pub enum ConnectionClose {
    GameEnded,
    NotFound,

    GameFull,
    TimedOut,

    ServerError,
    ServerRestart,
}

impl ToString for ConnectionClose {
    fn to_string(&self) -> String {
        self.get().1
    }
}

impl ConnectionClose {
    pub fn get(&self) -> (u16, String) {
        let result = match self {
            Self::GameEnded => (1000, "GAME_ENDED"),
            Self::NotFound => (4000, "NOT_FOUND"),

            Self::GameFull => (4100, "GAME_FULL"),
            Self::TimedOut => (3008, "TIMED_OUT"),

            Self::ServerError => (1011, "SERVER_ERROR"),
            Self::ServerRestart => (1012, "SERVER_RESTART"),
        };

        (result.0, result.1.to_string())
    }
    pub fn to_message(&self) -> Message {
        let (code, reason) = self.get();

        Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        }))
    }
}
