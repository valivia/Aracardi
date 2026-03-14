use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub title: Option<String>,
    pub text: String,
    pub image: bool,
    pub players: Vec<String>,
    pub turns: Option<i32>,
    pub time_limit: Option<u32>,
}
