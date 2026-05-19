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
    #[serde(default)]
    pub is_hand_picked: bool,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub was_loaded: bool,
}

impl Player {
    pub fn is_valid(&self) -> bool {
        let valid_name = (MIN_PLAYER_NAME_LENGTH..MAX_PLAYER_NAME_LENGTH)
            .contains(&self.name.encode_utf16().count());

        let valid_avatar =
            (MIN_PLAYER_AVATAR_LENGTH..MAX_PLAYER_AVATAR_LENGTH).contains(&self.avatar.len());

        let valid_id = self.id.len() == PLAYER_ID_LENGTH;

        valid_name && valid_avatar && valid_id
    }
}
