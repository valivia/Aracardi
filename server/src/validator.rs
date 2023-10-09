use crate::model::{
    card::Card,
    player_selector::{PlayerSelector, RangeSelector},
    stage::Stage,
};

pub fn validate_card(card: Card) -> Result<(), String> {
    let mut player_range = RangeSelector::new(None, None);

    for stage in card.stages {
        // Calculate player range
        let stage_range = calculate_stage_player_count(&stage)?;
        player_range = player_range.combine_stages(&stage_range);

        validate_substage_order(stage)?;
    }

    println!(
        "card Range ------\nmin: {:?}\nmax: {:?}",
        player_range.min, player_range.max
    );

    Ok(())
}

/// Validate that the sub stages are in the correct order
fn validate_substage_order(stage: Stage) -> Result<(), String> {
    for (index, sub_stage) in stage.sub_stages.iter().enumerate() {
        match &sub_stage.player_selector {
            PlayerSelector::Host => {
                if stage.sub_stages.iter().enumerate().any(|(i, entry)| {
                    i != index
                        && matches!(
                            entry.player_selector,
                            PlayerSelector::Host | PlayerSelector::Offset(_)
                        )
                }) {
                    return Err("Host can not be used alongside with offset".to_string());
                }
            }
            PlayerSelector::Offset(_) => {
                if stage.sub_stages.iter().enumerate().any(|(i, entry)| {
                    i != index
                        && matches!(entry.player_selector, PlayerSelector::Offset(_))
                        && entry.player_selector == sub_stage.player_selector
                }) {
                    return Err(
                        "Can not have multiple offset selectors with the same number".to_string(),
                    );
                }
            }
            PlayerSelector::Fill(range) | PlayerSelector::Random(range) => {
                // Make sure uncapped selectors are at the end of the vec.
                if range.max.is_none() && index != stage.sub_stages.len() - 1 {
                    return Err(
                        "An uncapped fill/random can only be used at the end of a stage"
                            .to_string(),
                    );
                }

                // Make sure fill/random selectors are after the mono selectors
                let mono_index = stage.sub_stages.iter().rev().position(|entry| {
                    matches!(
                        entry.player_selector,
                        PlayerSelector::Host | PlayerSelector::Offset(_)
                    )
                });

                if mono_index.unwrap_or(index) < index {
                    return Err(
                        "Fill and random can only be used after the mono selectors".to_string()
                    );
                }
            }
        }
    }

    Ok(())
}

/// Calculate the minimum and maximum number of players required for a stage
fn calculate_stage_player_count(stage: &Stage) -> Result<RangeSelector, String> {
    let mut stage_range = RangeSelector::new(None, Some(1));

    for sub_stage in &stage.sub_stages {
        let range: RangeSelector = match &sub_stage.player_selector {
            PlayerSelector::Host | PlayerSelector::Offset(_) => {
                RangeSelector::new(Some(1), Some(1))
            }
            PlayerSelector::Fill(range) => range.clone(),
            PlayerSelector::Random(range) => range.clone(),
        };

        stage_range = stage_range.combine_substages(&range);
    }

    println!(
        "Stage Range ------\nmin: {:?}\nmax: {:?}",
        stage_range.min, stage_range.max
    );

    Ok(stage_range)
}
