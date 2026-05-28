use super::{
    replay::GameplayReplayCheckpoint, GameplayRuntime, GameplayRuntimeError, GameplaySession,
};

pub fn recover_gameplay_session(
    session: GameplaySession,
    checkpoint: GameplayReplayCheckpoint,
    score: u64,
) -> Result<GameplayRuntime, GameplayRuntimeError> {
    GameplayRuntime::restore(session, checkpoint, score)
}
