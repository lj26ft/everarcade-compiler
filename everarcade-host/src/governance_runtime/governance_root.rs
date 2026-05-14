pub type Hash = [u8; 32];
use sha2::{Digest, Sha256};

use super::execution_window::GovernanceExecutionWindow;

pub fn governance_runtime_root(lineage_root: Hash, window: &GovernanceExecutionWindow) -> Hash {
    let mut h = Sha256::new();
    h.update(lineage_root);
    h.update(window.window_root);
    h.update(window.proposal_root);
    h.update(window.quorum_root);
    h.update(window.execution_scope_root);
    h.finalize().into()
}
