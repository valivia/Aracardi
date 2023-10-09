use super::stage::Stage;

#[derive(Debug, Clone)]
pub struct Card {
    pub is_offline_compatible: bool,
    pub is_online_compatible: bool,
    pub stages: Vec<Stage>,
}

impl Card {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self {
            stages,
            is_offline_compatible: true,
            is_online_compatible: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveCard {
    pub turns: usize,
    pub message: String,
}
