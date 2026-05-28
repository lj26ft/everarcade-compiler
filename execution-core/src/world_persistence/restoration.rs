use super::{checkpoint::WorldCheckpoint, validation::validate_checkpoint};
pub fn restore_checkpoint(cp: &WorldCheckpoint) -> Result<WorldCheckpoint, &'static str> {
    if validate_checkpoint(cp) {
        Ok(cp.clone())
    } else {
        Err("corrupted restoration rejected")
    }
}
