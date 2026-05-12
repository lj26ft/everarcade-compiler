use super::{checkpoint_snapshot::CheckpointSnapshot, checkpoint_validation::validate_checkpoint};

pub fn restore_checkpoint(snapshot: &CheckpointSnapshot) -> Result<Vec<u8>, String> {
    if !validate_checkpoint(snapshot) {
        return Err("checkpoint root mismatch".to_string());
    }
    Ok(snapshot.encoded_state.clone())
}
