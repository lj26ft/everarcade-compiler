use super::{GameplayReplayCheckpoint, GameplayReplayWindow};
use crate::gameplay::GameplayRuntimeError;

pub fn validate_append_only(
    previous_end: u64,
    window: &GameplayReplayWindow,
) -> Result<(), GameplayRuntimeError> {
    if window.start_tick != previous_end {
        return Err(GameplayRuntimeError::ReplayMutation);
    }
    if window.end_tick < window.start_tick {
        return Err(GameplayRuntimeError::ReplayCorruption);
    }
    Ok(())
}

pub fn validate_checkpoint(
    window: &GameplayReplayWindow,
    checkpoint: &GameplayReplayCheckpoint,
) -> Result<(), GameplayRuntimeError> {
    if window.continuity_root != checkpoint.continuity_root
        || window.state_root != checkpoint.state_root
        || window.end_tick != checkpoint.tick
    {
        return Err(GameplayRuntimeError::ReplayCorruption);
    }
    Ok(())
}
