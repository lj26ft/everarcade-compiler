use super::{runtime::AutonomousWorldRecoveryState, validation::equivalent};

pub fn restore(
    checkpoint: &AutonomousWorldRecoveryState,
    replay: &AutonomousWorldRecoveryState,
) -> Result<AutonomousWorldRecoveryState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
