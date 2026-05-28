use super::GameplayReplayCheckpoint;
use crate::gameplay::state::GameplayState;
use crate::gameplay::GameplayRuntimeError;

pub fn restore_checkpoint(
    checkpoint: &GameplayReplayCheckpoint,
    score: u64,
) -> Result<GameplayState, GameplayRuntimeError> {
    let restored = GameplayState {
        tick: checkpoint.tick,
        score,
        continuity_root: checkpoint.continuity_root.clone(),
        state_root: checkpoint.state_root.clone(),
    };
    if crate::gameplay::state::root_for(restored.tick, restored.score, &restored.continuity_root)
        != restored.state_root
    {
        return Err(GameplayRuntimeError::Divergence);
    }
    Ok(restored)
}
