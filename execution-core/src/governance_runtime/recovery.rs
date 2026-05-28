use super::{runtime::GovernanceRuntimeState, validation::equivalent};

pub fn restore(
    checkpoint: &GovernanceRuntimeState,
    replay: &GovernanceRuntimeState,
) -> Result<GovernanceRuntimeState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
