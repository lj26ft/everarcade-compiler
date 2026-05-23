use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticManifest {
    pub replay_root_a: String,
    pub replay_root_b: String,
    pub checkpoint_root_a: String,
    pub checkpoint_root_b: String,
    pub settlement_root: String,
    pub topology_root: String,
    pub continuity_summary: String,
}

impl DiagnosticManifest {
    pub fn report_root(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("diagnostics serialization must succeed"))
    }

    pub fn has_divergence(&self) -> bool {
        self.replay_root_a != self.replay_root_b || self.checkpoint_root_a != self.checkpoint_root_b
    }
}
