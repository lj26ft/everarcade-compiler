use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSession {
    pub session_id: Hash,
    pub parent_session: Option<Hash>,
    pub replay_root: Hash,
    pub convergence_root: Hash,
}
