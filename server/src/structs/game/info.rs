use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub addons: Vec<String>,

    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(rename = "initiatedAtMs")]
    pub initiated_at: DateTime<Utc>,

    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(rename = "startedAtMs")]
    // TODO: Remove default when sufficient people have updated
    #[serde(default = "get_utc")]
    pub started_at: DateTime<Utc>,

    #[serde_as(serialize_as = "TimestampMilliSeconds<i64>")]
    #[serde(skip_deserializing, default = "get_utc")]
    pub ended_at: DateTime<Utc>,
    pub version: String,
}

fn get_utc() -> DateTime<Utc> {
    Utc::now()
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            addons: vec![],
            started_at: Utc::now(),
            ended_at: Utc::now(),
            initiated_at: Utc::now(),
            version: "".to_string(),
        }
    }
}
