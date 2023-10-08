use super::{player::Player, sub_stage::SubStage};

#[derive(Debug, Clone, Default)]
pub enum StageState {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub state: StageState,
    pub time_limit: Option<u32>,
    pub sub_stages: Vec<SubStage>,
}

impl Stage {
    pub fn new(sub_stages: Vec<SubStage>) -> Self {
        Self {
            state: StageState::NotStarted,
            time_limit: None,
            sub_stages,
        }
    }

    pub fn assign_players(
        &mut self,
        players: &[Player],
        current_player: Player,
    ) -> Result<(), String> {
        let mut available_players = players.to_vec();

        for sub_stage in &mut self.sub_stages {
            // Assign players to sub stage
            let targets = sub_stage
                .assign_players(&available_players, current_player.clone())
                .unwrap();

            // Remove assigned players from available players
            for player in targets {
                available_players
                    .iter()
                    .position(|p| p.id == player.id)
                    .map(|i| available_players.remove(i));
            }
        }

        // TODO, return emptyStage if leftover players?
        if available_players.is_empty() {
            Ok(())
        } else {
            Err("Leftover players".to_string())
        }
    }
}
