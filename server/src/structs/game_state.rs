use crate::structs::player::PlayerId;

pub struct GameState {
    pub players: Vec<PlayerId>,
    pub current_player: Option<PlayerId>,
    pub current_card: Option<String>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            players: Vec::new(),
            current_player: None,
            current_card: None,
        }
    }
}
