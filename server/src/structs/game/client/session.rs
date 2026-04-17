use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Serialize)]
pub struct SessionMeta {
    pub original_connect_at: DateTime<Utc>,

    #[serde(skip)]
    pub last_reconnect_at: Option<DateTime<Utc>>,

    pub total_time_connected_s: i64,

    pub reconnect_count: u32,
    pub disconnect_count: u32,
}

impl SessionMeta {
    pub fn new() -> Self {
        Self {
            original_connect_at: Utc::now(),
            last_reconnect_at: None,

            total_time_connected_s: 0,

            reconnect_count: 0,
            disconnect_count: 0,
        }
    }

    pub fn log_disconnect(&mut self) {
        self.disconnect_count += 1;
        if let Some(last_reconnect) = self.last_reconnect_at {
            self.total_time_connected_s += (Utc::now() - last_reconnect).num_seconds();
            self.last_reconnect_at = None;
        } else if self.total_time_connected_s == 0 {
            self.total_time_connected_s += (Utc::now() - self.original_connect_at).num_seconds();
        }
    }

    pub fn log_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.last_reconnect_at = Some(Utc::now());
    }
}
