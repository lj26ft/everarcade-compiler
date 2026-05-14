use sha2::{Digest, Sha256};

use super::{execution_commit::ExecutionCommit, execution_lineage::ExecutionLineage};

pub type Hash = [u8; 32];

pub fn compute_distributed_execution_root(
    commit: &ExecutionCommit,
    lineage: &ExecutionLineage,
) -> Hash {
    let mut h = Sha256::new();
    h.update(commit.window_id);
    h.update(commit.assignment_id);
    h.update(commit.execution_root);
    h.update(commit.receipt_root);
    h.update(lineage.lineage_root);
    h.update(lineage.assignment_id);
    h.update(lineage.checkpoint_root);
    if let Some(parent) = lineage.parent_lineage {
        h.update(parent);
    }
    h.finalize().into()
}
