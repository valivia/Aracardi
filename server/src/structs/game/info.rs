use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct GameInfo {
    pub addons: Vec<String>,
    pub theme: String,
    pub setup_time_ms: i64,
    pub version: String,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            addons: vec![],
            theme: "".to_string(),
            setup_time_ms: 0,
            version: "".to_string(),
        }
    }
}
