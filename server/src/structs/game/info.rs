use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub addons: Vec<String>,

    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub initiated_at_ms: DateTime<Utc>,

    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(skip_deserializing, default = "get_utc")]
    pub started_at_ms: DateTime<Utc>,

    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(skip_deserializing, default = "get_utc")]
    pub ended_at_ms: DateTime<Utc>,
    pub version: String,
}

fn get_utc() -> DateTime<Utc> {
    Utc::now()
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            addons: vec![],
            started_at_ms: Utc::now(),
            ended_at_ms: Utc::now(),
            initiated_at_ms: Utc::now(),
            version: "".to_string(),
        }
    }
}
