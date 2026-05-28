use crate::gameplay::{replay::GameplayReplayCheckpoint, GameplayRuntimeError};
pub fn validate_checkpoint_restore(
    checkpoint: &GameplayReplayCheckpoint,
    continuity_root: &str,
) -> Result<(), GameplayRuntimeError> {
    if checkpoint.continuity_root == continuity_root && checkpoint.state_root.starts_with("sha256:")
    {
        Ok(())
    } else {
        Err(GameplayRuntimeError::InvalidRestoration)
    }
}
