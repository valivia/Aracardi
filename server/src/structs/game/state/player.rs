use serde::{Deserialize, Serialize};

static MAX_PLAYER_NAME_LENGTH: u8 = 20;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub avatar: String,

    #[serde(skip_serializing)]
    pub is_hand_picked: bool,
}
