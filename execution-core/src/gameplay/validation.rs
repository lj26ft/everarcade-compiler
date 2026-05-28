use super::{GameplayRuntime, GameplayRuntimeError};

pub fn validate_runtime(runtime: &GameplayRuntime) -> Result<(), GameplayRuntimeError> {
    if !runtime.continuity.append_only {
        return Err(GameplayRuntimeError::ReplayMutation);
    }
    if runtime.session.continuity_root != runtime.world.state.continuity_root {
        return Err(GameplayRuntimeError::Divergence);
    }
    Ok(())
}
