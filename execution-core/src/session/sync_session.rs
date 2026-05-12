use serde::{Deserialize, Serialize};

use super::execution_session::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSession {
    pub session_id: Hash,
    pub replay_root: Hash,
    pub synchronized: bool,
}
