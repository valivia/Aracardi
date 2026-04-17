use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Serialize)]
pub struct SessionData {
    pub original_connect_at: DateTime<Utc>,
    pub is_connected: bool,

    #[serde(skip)]
    pub last_reconnect_at: Option<DateTime<Utc>>,
    pub reconnect_count: u32,

    pub total_time_connected_s: i64,
}

impl SessionData {
    pub fn new() -> Self {
        Self {
            original_connect_at: Utc::now(),
            is_connected: true,

            last_reconnect_at: None,
            reconnect_count: 0,

            total_time_connected_s: 0,
        }
    }

    pub fn log_disconnect(&mut self) {
        if let Some(last_reconnect) = self.last_reconnect_at {
            self.total_time_connected_s += (Utc::now() - last_reconnect).num_seconds();
            self.last_reconnect_at = None;
        } else if self.total_time_connected_s == 0 {
            self.total_time_connected_s += (Utc::now() - self.original_connect_at).num_seconds();
        }
        self.is_connected = false;
    }

    pub fn log_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.last_reconnect_at = Some(Utc::now());
        self.is_connected = true;
    }
}
