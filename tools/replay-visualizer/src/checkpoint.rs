use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointView { pub tick: u64, pub checkpoint_root: String }

pub fn inspect_checkpoint(tick: u64, state_root: &str) -> CheckpointView { CheckpointView { tick, checkpoint_root: stable_hash(&["checkpoint", state_root]) } }
