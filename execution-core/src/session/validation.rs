use crate::gameplay::{GameplayRuntime, GameplayRuntimeError};
pub fn validate_session_equivalence(
    left: &GameplayRuntime,
    right: &GameplayRuntime,
) -> Result<(), GameplayRuntimeError> {
    if left.world.state == right.world.state
        && left.session.continuity_root == right.session.continuity_root
    {
        Ok(())
    } else {
        Err(GameplayRuntimeError::Divergence)
    }
}
