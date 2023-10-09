use super::stage::Stage;

#[derive(Debug, Clone)]
pub struct Card {
    pub stages: Vec<Stage>,
}

impl Card {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveCard {
    pub turns: usize,
    pub message: String,
}