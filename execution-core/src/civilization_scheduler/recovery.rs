use super::{runtime::CivilizationSchedulerState, validation::equivalent};

pub fn restore(
    checkpoint: &CivilizationSchedulerState,
    replay: &CivilizationSchedulerState,
) -> Result<CivilizationSchedulerState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
