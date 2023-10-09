use crate::model::{
    card::Card,
    player_selector::{PlayerSelector, RangeSelector},
    stage::Stage,
    sub_stage::SubStageType,
};

pub fn validate_card(card: &Card) -> Result<(), String> {
    let mut combined_player_range = RangeSelector::new(None, None);

    for stage in &card.stages {
        if stage.sub_stages.is_empty() {
            return Err("Substages can not be empty".to_string());
        }

        if card.is_offline_compatible {
            validate_offline_compatibility(stage)?;
        }

        validate_player_selectors(stage)?;

        // Calculate player range
        let stage_range = calculate_stage_player_count(stage)?;
        combined_player_range = combined_player_range.intersect(&stage_range);
    }

    println!(
        "card Range ------\nmin: {:?}\nmax: {:?}",
        combined_player_range.min, combined_player_range.max
    );

    Ok(())
}

/// Validate that the stage is offline mode compatible
pub fn validate_offline_compatibility(stage: &Stage) -> Result<(), String> {
    if stage.sub_stages.len() > 1 {
        return Err("Offline compatible cards can only have one sub stage per stage".to_string());
    }

    let has_incompatible_substage = stage
        .sub_stages
        .iter()
        .any(|sub_stage| !matches!(sub_stage.sub_type, SubStageType::Text(_)));
    if has_incompatible_substage {
        return Err("Offline compatible cards can only have text sub stages".to_string());
    }

    match &stage
        .sub_stages
        .first()
        .expect("Already checked if not empty")
        .player_selector
    {
        PlayerSelector::Fill(_) | PlayerSelector::Random(_) => Ok(()),
        _ => Err("Offline cards can not use mono selectors".to_string()),
    }
}

/// Validate that the sub stages are in the correct order with respect to the player selectors
fn validate_player_selectors(stage: &Stage) -> Result<(), String> {
    for (index, sub_stage) in stage.sub_stages.iter().enumerate() {
        match &sub_stage.player_selector {
            PlayerSelector::Host => {
                if stage
                    .sub_stages
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .any(|(_, entry)| {
                        matches!(
                            entry.player_selector,
                            PlayerSelector::Host | PlayerSelector::Offset(_)
                        )
                    })
                {
                    return Err("Host can not be used alongside with offset".to_string());
                }
            }
            PlayerSelector::Offset(offset) => {
                (!stage
                    .sub_stages
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .any(|(_, entry)| match &entry.player_selector {
                        PlayerSelector::Offset(other_offset) => offset == other_offset,
                        _ => false,
                    }))
                .then_some(())
                .ok_or("Can not have multiple offset selectors with the same number".to_string())?;
            }
            PlayerSelector::Fill(range) | PlayerSelector::Random(range) => {
                // Make sure uncapped selectors are at the end of the substage list.
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

/// Calculate the range of players allowed for a stage
fn calculate_stage_player_count(stage: &Stage) -> Result<RangeSelector, String> {
    // the starting point for the combining "maths"
    let mut stage_range = RangeSelector::new(None, Some(1));

    for sub_stage in &stage.sub_stages {
        let range: RangeSelector = match &sub_stage.player_selector {
            PlayerSelector::Host | PlayerSelector::Offset(_) => {
                RangeSelector::new(Some(1), Some(1))
            }
            PlayerSelector::Fill(range) => *range,
            PlayerSelector::Random(range) => *range,
        };

        stage_range = stage_range.combine_substages(&range);
    }

    println!(
        "Stage Range ------\nmin: {:?}\nmax: {:?}",
        stage_range.min, stage_range.max
    );

    Ok(stage_range)
}
