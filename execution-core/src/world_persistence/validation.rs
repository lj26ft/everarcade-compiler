use super::checkpoint::WorldCheckpoint;
pub fn validate_checkpoint(c: &WorldCheckpoint) -> bool {
    c.checkpoint_root == format!("checkpoint:{}:{}:{}", c.tick, c.world_root, c.replay_tip)
}
pub fn reject_non_append_only(old_len: usize, new_len: usize) -> Result<(), &'static str> {
    if new_len >= old_len {
        Ok(())
    } else {
        Err("non append-only restoration rejected")
    }
}
