use super::stage::Stage;

#[derive(Debug, Clone)]
pub struct Card {
    pub is_offline_compatible: bool,
    pub is_online_compatible: bool,

    pub min_players: Option<usize>,
    pub max_players: Option<usize>,

    pub stages: Vec<Stage>,
}

impl Card {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self {
            is_offline_compatible: true,
            is_online_compatible: true,

            min_players: None,
            max_players: None,

            stages,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveCard {
    pub turns: usize,
    pub message: String,
}
