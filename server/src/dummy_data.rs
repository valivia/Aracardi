use crate::model::{
    card::Card,
    player_selector::{PlayerSelector, RangeSelector},
    stage::{Stage, StageState},
    sub_stage::{PollSubStage, SubStage, SubStageType, TextSubStage},
};

use crate::validator::validate_card;

// Cards
pub fn get_cards() -> Vec<Card> {
    let mut cards = vec![
        Card {
            stages: vec![
                Stage {
                    state: StageState::NotStarted,
                    time_limit: None,
                    sub_stages: vec![
                        SubStage {
                            player_selector: PlayerSelector::Random(RangeSelector::new(
                                Some(1),
                                Some(1),
                            )),
                            targets: vec![],

                            sub_type: SubStageType::Poll(PollSubStage {
                                title: "Choose your favorite animal".to_string(),
                                options: vec!["A".to_string(), "B".to_string()],
                            }),
                        },
                        SubStage {
                            player_selector: PlayerSelector::Fill(RangeSelector::new(None, None)),
                            targets: vec![],

                            sub_type: SubStageType::Poll(PollSubStage {
                                title: "Guess what {card.0.0.target}'s favourite animal is"
                                    .to_string(),
                                options: vec!["A".to_string(), "B".to_string()],
                            }),
                        },
                    ],
                },
                Stage {
                    state: StageState::NotStarted,
                    time_limit: None,
                    sub_stages: vec![SubStage {
                        player_selector: PlayerSelector::Fill(RangeSelector::new(None, None)),
                        targets: vec![],

                        sub_type: SubStageType::Text(TextSubStage {
                            title: None,
                            text: "{card.0.0.target} chose {card.0.0.result}".to_string(),
                        }),
                    }],
                },
            ],
        },
        Card {
            stages: vec![Stage {
                state: StageState::NotStarted,
                time_limit: None,
                sub_stages: vec![
                    SubStage {
                        player_selector: PlayerSelector::Offset(0),
                        targets: vec![],

                        sub_type: SubStageType::Text(TextSubStage {
                            title: Some("Friends".to_string()),
                            text: "Test".to_string(),
                        }),
                    },
                    SubStage {
                        player_selector: PlayerSelector::Fill(RangeSelector::new(Some(2), Some(4))),
                        targets: vec![],

                        sub_type: SubStageType::Text(TextSubStage {
                            title: Some("Friends".to_string()),
                            text: "Test".to_string(),
                        }),
                    },
                    SubStage {
                        player_selector: PlayerSelector::Random(RangeSelector::new(None, None)),
                        targets: vec![],

                        sub_type: SubStageType::Text(TextSubStage {
                            title: Some("Friends".to_string()),
                            text: "Drink if you've met {random}".to_string(),
                        }),
                    },
                ],
            }],
        },
    ];

    for card in cards.clone() {
        validate_card(card).unwrap();
    }

    for _ in 0..3 {
        let new_cards = cards.to_vec();
        cards.extend(new_cards);
    }

    cards
}
