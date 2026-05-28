use super::{runtime::CivilizationInteractionState, validation::equivalent};

pub fn restore(
    checkpoint: &CivilizationInteractionState,
    replay: &CivilizationInteractionState,
) -> Result<CivilizationInteractionState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
