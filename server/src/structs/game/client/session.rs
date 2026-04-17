use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SessionMeta {
    pub original_connect_at: DateTime<Utc>,
    #[serde(skip)]
    pub last_reconnect_at: Option<DateTime<Utc>>,

    pub total_connected_duration: Duration,

    pub reconnect_count: u32,
    pub disconnect_count: u32,
}

impl SessionMeta {
    pub fn new() -> Self {
        Self {
            original_connect_at: Utc::now(),
            last_reconnect_at: None,

            total_connected_duration: Duration::default(),

            reconnect_count: 0,
            disconnect_count: 0,
        }
    }

    pub fn log_disconnect(&mut self) {
        self.disconnect_count += 1;
        if let Some(last_reconnect) = self.last_reconnect_at {
            self.total_connected_duration += Utc::now().signed_duration_since(last_reconnect);
            self.last_reconnect_at = None;
        } else if self.total_connected_duration == Duration::zero() {
            self.total_connected_duration +=
                Utc::now().signed_duration_since(self.original_connect_at);
        }
    }

    pub fn log_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.last_reconnect_at = Some(Utc::now());
    }
}
