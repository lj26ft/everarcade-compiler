use super::{checkpoint_root::compute_checkpoint_root, checkpoint_snapshot::CheckpointSnapshot};

pub fn validate_checkpoint(snapshot: &CheckpointSnapshot) -> bool {
    compute_checkpoint_root(snapshot) == snapshot.checkpoint_root
}
