use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const MIN_PLAYER_NAME_LENGTH: usize = 3;
const MAX_PLAYER_NAME_LENGTH: usize = 20;

// TODO: possibly check the actual avatar list?
const MIN_PLAYER_AVATAR_LENGTH: usize = 3;
const MAX_PLAYER_AVATAR_LENGTH: usize = 20;

const PLAYER_ID_LENGTH: usize = 16;

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
        return (MIN_PLAYER_NAME_LENGTH..MAX_PLAYER_NAME_LENGTH).contains(&self.name.len())
            && (MIN_PLAYER_AVATAR_LENGTH..MAX_PLAYER_AVATAR_LENGTH).contains(&self.avatar.len())
            && self.id.len() == PLAYER_ID_LENGTH;
    }
}
