use super::{errors::StateError, tree::{CanonicalState, Hash256}};

pub fn encode_checkpoint(state: &CanonicalState) -> Result<Vec<u8>, StateError> { Ok(bincode::serialize(state)?) }

pub fn decode_checkpoint(bytes: &[u8]) -> Result<CanonicalState, StateError> { Ok(bincode::deserialize(bytes)?) }

pub fn decode_checkpoint_with_expected_root(bytes: &[u8], expected: Hash256) -> Result<CanonicalState, StateError> {
    let state = decode_checkpoint(bytes)?;
    let actual = state.root();
    if actual != expected { return Err(StateError::RootMismatch { expected, actual }); }
    Ok(state)
}
