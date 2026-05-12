use serde::{Deserialize, Serialize};

use super::retention_window::RetentionWindow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PruningPolicy {
    pub retention: RetentionWindow,
    pub require_checkpoint: bool,
    pub require_proof_commitment: bool,
}
