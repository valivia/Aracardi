use std::time::Duration;

use axum::http::HeaderMap;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct SessionMeta {
    #[serde(skip)]
    pub remote_addr: Option<String>,
    pub user_agent: Option<String>,
    pub country_code: Option<String>,

    pub original_connect_at: DateTime<Utc>,
    pub reconnect_count: u32,
    pub last_reconnect_at: Option<DateTime<Utc>>,
    pub total_connected_duration: Duration,
    pub disconnect_count: u32,
}

impl SessionMeta {
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            remote_addr: headers
                .get("CF-Connecting-IP")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
            user_agent: headers
                .get("user_agent")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
            country_code: headers
                .get("CF-IPCountry")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),

            original_connect_at: Utc::now(),
            total_connected_duration: Duration::default(),

            last_reconnect_at: None,
            reconnect_count: 0,
            disconnect_count: 0,
        }
    }
}
