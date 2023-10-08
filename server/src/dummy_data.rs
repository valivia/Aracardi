use crate::model::{
    card::Card,
    stage::{Stage, StageState},
    sub_stage::{PollSubStage, SubStage, TextSubStage},
};

// Cards
pub fn get_cards() -> Vec<Card> {
    vec![]
    // let mut cards = vec![
    //     Card {
    //         stages: vec![
    //             Stage {
    //                 state: StageState::NotStarted,
    //                 time_limit: None,
    //                 sub_stages: vec![
    //                     SubStage::Poll(PollSubStage {
    //                         player_selector: "random".to_string(),
    //                         targets: vec![],

    //                         title: "Choose your favorite animal".to_string(),
    //                         options: vec!["A".to_string(), "B".to_string()],
    //                     }),
    //                     SubStage::Poll(PollSubStage {
    //                         player_selector: "fill".to_string(),
    //                         targets: vec![],

    //                         title: "Guess what {card.0.0.target}'s favourite animal is".to_string(),
    //                         options: vec!["A".to_string(), "B".to_string()],
    //                     }),
    //                 ],
    //             },
    //             Stage {
    //                 state: StageState::NotStarted,
    //                 time_limit: None,
    //                 sub_stages: vec![SubStage::Text(TextSubStage {
    //                     player_selector: "fill".to_string(),
    //                     targets: vec![],

    //                     title: None,
    //                     text: "{card.0.0.target} chose {card.0.0.result}".to_string(),
    //                 })],
    //             },
    //         ],
    //     },
    //     Card {
    //         stages: vec![Stage {
    //             state: StageState::NotStarted,
    //             time_limit: None,
    //             sub_stages: vec![SubStage::Text(TextSubStage {
    //                 player_selector: "fill".to_string(),
    //                 targets: vec![],
    //                 title: Some("Friends".to_string()),
    //                 text: "Drink if you've met {random}".to_string(),
    //             })],
    //         }],
    //     },
    // ];

    // for _ in 0..3 {
    //     let new_cards = cards.to_vec();
    //     cards.extend(new_cards);
    // }

    // cards
}
