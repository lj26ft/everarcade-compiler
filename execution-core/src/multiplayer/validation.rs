#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultiplayerError {
    DivergentInput,
    InvalidFrameOrdering,
}
use super::input::PlayerInput;
pub fn reject_divergent_inputs(
    left: &[PlayerInput],
    right: &[PlayerInput],
) -> Result<(), MultiplayerError> {
    if left == right {
        Ok(())
    } else {
        Err(MultiplayerError::DivergentInput)
    }
}
