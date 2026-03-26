use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub avatar: String,

    #[serde(skip_serializing)]
    pub is_hand_picked: Option<bool>,
    #[serde(skip_serializing)]
    pub was_loaded: Option<bool>,
}

impl Player {
    pub fn is_valid(&self) -> bool {
        return (3..20).contains(&self.name.len())
            && (3..20).contains(&self.avatar.len())
            && self.id.len() == 16;
    }
}
