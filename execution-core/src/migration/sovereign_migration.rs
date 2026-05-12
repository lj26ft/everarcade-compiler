use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SovereignMigration {
    pub migration_id: Hash,
    pub source_domain: Hash,
    pub target_domain: Hash,
    pub continuity_root: Hash,
    pub checkpoint_root: Hash,
}
