use crate::coordination::upgrade::ProtocolUpgradeManifest;
use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationTransition {
    pub replay_transition: String,
    pub capability_transition: String,
    pub topology_transition: String,
    pub checkpoint_transition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationJournal {
    pub upgrades: Vec<ProtocolUpgradeManifest>,
    pub transitions: Vec<MigrationTransition>,
}

impl MigrationJournal {
    pub fn append_upgrade(
        &mut self,
        manifest: ProtocolUpgradeManifest,
        transition: MigrationTransition,
    ) {
        self.upgrades.push(manifest);
        self.transitions.push(transition);
    }

    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("migration journal serialize failed"))
    }
}
