use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub addons: Vec<String>,
    pub initiated_at_ms: u32,
    #[serde(skip_deserializing)]
    pub started_at_ms: i64,
    #[serde(skip_deserializing, default = "now_in_ms")]
    pub ended_at_ms: i64,
    pub version: String,
}

fn now_in_ms() -> i64 {
    Utc::now().timestamp_millis()
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            addons: vec![],
            started_at_ms: 0,
            ended_at_ms: 0,
            initiated_at_ms: 0,
            version: "".to_string(),
        }
    }
}
