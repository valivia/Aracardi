use model::{
    card::{ActiveCard, Card},
    player::Player,
};
use rand::Rng;

mod dummy_data;
use crate::dummy_data::get_cards;

mod model;

fn main() {
    println!("Hello, world!");
    let mut game = OfflineGame::new(
        get_cards(),
        (0..5)
            .map(|x| Player {
                id: format!("{}", x),
                name: format!("{}", x),
                avatar: String::new(),
                host: false,
                score: 0,
            })
            .collect::<Vec<Player>>(),
    )
    .unwrap();

    for _ in 0..10 {
        game.next();
    }
}

#[derive(Debug, Clone)]
struct GameState {
    player: Player,
    card: Card,
    card_index: usize,
    stage_index: usize,
}

#[derive(Debug, Clone)]
pub struct OfflineGame {
    state: GameState,
    active_cards: Vec<ActiveCard>,
    
    /// When to start reusing cards. Has to be less than cards.len()
    backlog_limit: usize,

    players: Vec<Player>,

    cards: Vec<Card>,
}

impl OfflineGame {
    pub fn new(cards: Vec<Card>, players: Vec<Player>) -> Result<Self, String> {
        if cards.len() < 10 {
            return Err("Not enough cards".to_string());
        }

        if players.len() < 3 {
            return Err("Not enough players".to_string());
        }

        println!("cards: {:?}, players: {:?}", cards.len(), players.len());

        let state = GameState {
            player: players[0].clone(),
            card: cards[0].clone(),
            stage_index: 0,
            card_index: 0,
        };

        Ok(Self {
            players,
            backlog_limit: (0.8 * cards.len() as f32) as usize,
            cards,
            active_cards: Vec::new(),
            state,
        })
    }

    // Player management
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: Player) {
        self.players.retain(|p| p.id != player.id);
    }

    // Turn management

    /// Move to next turn
    fn next_turn(&mut self) {
        // Increment card pointer if below backlog limit
        if self.state.card_index < self.backlog_limit {
            self.state.card_index += 1;
        } else {
            // Move random previousCard to front
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..(self.backlog_limit as f64 * 0.4) as usize);
            let card = self.cards.remove(index);
            self.cards.push(card);
        }

        let card = self.cards[self.state.card_index].clone();

        let next_player_index = (self
            .players
            .iter()
            .position(|p| p.id == self.state.player.id)
            .unwrap()
            + 1)
            % self.players.len();

        self.state = GameState {
            player: self.players[next_player_index].clone(),
            card,
            stage_index: 0,
            card_index: self.state.card_index,
        };
    }

    /// Move to next stage
    fn next_stage(&mut self) {
        self.state.stage_index += 1;
    }

    pub fn next(&mut self) {
        self.next_stage();
        if self.state.stage_index >= self.state.card.stages.len() {
            self.next_turn();
        }

        dbg!(self.state.card.stages.len(), self.state.stage_index);

        self.state.card.stages[self.state.stage_index]
            .assign_players(&self.players, self.state.player.clone())
            .unwrap();

        println!(
            "cardPointer: {}, stagePointer: {}",
            self.state.card_index, self.state.stage_index
        );
    }
}
