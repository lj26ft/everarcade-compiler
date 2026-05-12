use sha2::{Digest, Sha256};

use super::execution_session::{ExecutionSession, Hash};

pub fn compute_session_id(parent_session: Option<Hash>, replay_root: Hash, convergence_root: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(parent_session.unwrap_or([0; 32]));
    hasher.update(replay_root);
    hasher.update(convergence_root);
    hasher.finalize().into()
}

pub fn materialize_session(parent_session: Option<Hash>, replay_root: Hash, convergence_root: Hash) -> ExecutionSession {
    let session_id = compute_session_id(parent_session, replay_root, convergence_root);
    ExecutionSession { session_id, parent_session, replay_root, convergence_root }
}
