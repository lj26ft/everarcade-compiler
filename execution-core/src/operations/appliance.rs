use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplianceManifest {
    pub runtime_package_hash: String,
    pub deployment_topology_hash: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub orchestration_root: String,
    pub persistence_root: String,
}

impl ApplianceManifest {
    pub fn new(
        runtime_package_hash: String,
        deployment_topology_hash: String,
        replay_root: String,
        checkpoint_root: String,
        persistence_root: String,
    ) -> Self {
        let orchestration_root = hash_bytes(
            &bincode::serialize(&(
                &runtime_package_hash,
                &deployment_topology_hash,
                &replay_root,
                &checkpoint_root,
                &persistence_root,
            ))
            .expect("appliance manifest serialization must succeed"),
        );
        Self {
            runtime_package_hash,
            deployment_topology_hash,
            replay_root,
            checkpoint_root,
            orchestration_root,
            persistence_root,
        }
    }

    pub fn verify_orchestration_root(&self) -> bool {
        self.orchestration_root
            == hash_bytes(
                &bincode::serialize(&(
                    &self.runtime_package_hash,
                    &self.deployment_topology_hash,
                    &self.replay_root,
                    &self.checkpoint_root,
                    &self.persistence_root,
                ))
                .expect("appliance verification serialization must succeed"),
            )
    }
}
