use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};

#[serde_as]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub original_connect_at: DateTime<Utc>,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    pub disconnected_at: Option<DateTime<Utc>>,

    #[serde(skip)]
    pub last_reconnect_at: Option<DateTime<Utc>>,
    pub reconnect_count: u32,

    pub total_time_connected_s: i64,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            original_connect_at: Utc::now(),
            disconnected_at: None,

            last_reconnect_at: None,
            reconnect_count: 0,

            total_time_connected_s: 0,
        }
    }
}

impl SessionData {
    pub fn log_disconnect(&mut self, game_ended: bool) {
        if let Some(last_reconnect) = self.last_reconnect_at {
            self.total_time_connected_s += (Utc::now() - last_reconnect).num_seconds();
            self.last_reconnect_at = None;
        } else if self.total_time_connected_s == 0 {
            self.total_time_connected_s += (Utc::now() - self.original_connect_at).num_seconds();
        }

        if !game_ended {
            self.disconnected_at = Some(Utc::now());
        }
    }

    pub fn log_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.last_reconnect_at = Some(Utc::now());
        self.disconnected_at = None;
    }
}
