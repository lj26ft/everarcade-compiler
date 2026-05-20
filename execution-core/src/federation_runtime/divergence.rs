use serde::{Deserialize, Serialize};

use super::bundle::ContinuityBundle;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DivergenceKind {
    CheckpointMismatch,
    ReceiptMismatch,
    ExecutionHashMismatch,
    JournalDivergence,
    StateRootDivergence,
    ReplayDivergence,
}

pub fn compare_continuity_roots(local: [u8; 32], peer: [u8; 32]) -> bool {
    local == peer
}

pub fn verify_peer_replay(local: &ContinuityBundle, peer: &ContinuityBundle) -> bool {
    local.receipt_hashes == peer.receipt_hashes
        && local.execution_hashes == peer.execution_hashes
        && local.state_root == peer.state_root
}

pub fn detect_divergence(
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Option<DivergenceKind> {
    if local.checkpoint_hash != peer.checkpoint_hash {
        return Some(DivergenceKind::CheckpointMismatch);
    }
    if local.journal_hash != peer.journal_hash {
        return Some(DivergenceKind::JournalDivergence);
    }
    if local.receipt_hashes != peer.receipt_hashes {
        return Some(DivergenceKind::ReceiptMismatch);
    }
    if local.execution_hashes != peer.execution_hashes {
        return Some(DivergenceKind::ExecutionHashMismatch);
    }
    if local.state_root != peer.state_root {
        return Some(DivergenceKind::StateRootDivergence);
    }
    if local.continuity_hash != peer.continuity_hash {
        return Some(DivergenceKind::ReplayDivergence);
    }
    None
}
