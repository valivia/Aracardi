use rand::seq::SliceRandom;

use super::{
    player::{self, Player},
    player_selector::PlayerSelector,
};

#[derive(Debug, Clone)]
pub struct TextSubStage {
    pub title: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct InputSubStage {
    pub placeholder: String,
}

#[derive(Debug, Clone)]
pub struct PollSubStage {
    pub title: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EmptySubStage {}

#[derive(Debug, Clone)]
pub enum SubStageType {
    Text(TextSubStage),
    Input(InputSubStage),
    Poll(PollSubStage),
    Empty(EmptySubStage),
}

#[derive(Debug, Clone)]
pub struct SubStage {
    pub player_selector: PlayerSelector,
    pub targets: Vec<Player>,

    pub sub_type: SubStageType,
}

impl SubStage {
    pub fn set_targets(&mut self, players: Vec<Player>) {
        self.targets = players;
    }

    pub fn assign_players(
        &mut self,
        players: &[Player],
        current_player: Player,
    ) -> Result<Vec<Player>, String> {
        let mut rng = rand::thread_rng();
        let target_players = match &self.player_selector {
            PlayerSelector::Host => players
                .iter()
                .filter(|player| player.host)
                .cloned()
                .collect::<Vec<Player>>(),
            PlayerSelector::Current => vec![current_player.clone()],
            PlayerSelector::Offset(offset) => {
                let index = players
                    .iter()
                    .position(|player| player.id == current_player.id)
                    .unwrap();
                let mut players = players.to_vec();
                players.rotate_left(index + *offset as usize);
                players.truncate(1);
                players
            }
            PlayerSelector::Random(range) | PlayerSelector::Fill(range) => {
                if range.min.is_some_and(|min| min > players.len()) {
                    return Err("Not enough players".to_string());
                }

                let mut players = players.to_vec();
                if let PlayerSelector::Random(_) = self.player_selector {
                    players.shuffle(&mut rng);
                }

                if range.max.is_some_and(|max| max < players.len()) {
                    players.truncate(range.max.expect("We just checked, that it's Some"));
                }

                players
            }
        };

        self.targets = players.to_vec();

        Ok(target_players)
    }
}

// struct InputSubstage {
//     player_selector: String,
//     placeholder: String,
//     state: InputSubstageState,
// }

// enum InputSubstageState {
//     Start,
//     Loaded(Vec<Player>),
//     Answered(Vec<Player>, Vec<String>),
// }

// impl InputSubstageState {
//     fn start() -> Self {
//         InputSubstageState::Start
//     }

//     fn load(self, players: Vec<Player>) -> Self {
//         match self {
//             InputSubstageState::Start => InputSubstageState::Loaded(players),
//             _ => panic!("Invalid state"), // ideally you'd rather return a result or smth
//         }
//     }

//     fn answer(self, answers: Vec<String>) -> Self {
//         match self {
//             InputSubstageState::Loaded(players) => InputSubstageState::Answered(players, answers),
//             _ => panic!("Invalid state"), // ditto
//         }
//     }
// }
