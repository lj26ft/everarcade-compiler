use super::{input::PlayerInput, validation::MultiplayerError};
pub fn deterministic_order(
    mut inputs: Vec<PlayerInput>,
    frame: u64,
) -> Result<Vec<PlayerInput>, MultiplayerError> {
    if inputs.iter().any(|i| i.frame != frame) {
        return Err(MultiplayerError::InvalidFrameOrdering);
    }
    inputs.sort();
    Ok(inputs)
}
