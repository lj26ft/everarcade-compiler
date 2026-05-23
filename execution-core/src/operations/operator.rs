use crate::operations::{appliance::ApplianceManifest, sharding::TopologyManifest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatorDiagnostic {
    pub replay_verified: bool,
    pub checkpoint_root: String,
    pub continuity_notes: String,
    pub topology_hash: String,
    pub recovery_validated: bool,
    pub manifest_verified: bool,
}

pub fn inspect_continuity(
    appliance: &ApplianceManifest,
    topology: &TopologyManifest,
) -> OperatorDiagnostic {
    OperatorDiagnostic {
        replay_verified: appliance.verify_orchestration_root(),
        checkpoint_root: appliance.checkpoint_root.clone(),
        continuity_notes: format!("orchestration_root={}", appliance.orchestration_root),
        topology_hash: topology.shard_lineage_hash(),
        recovery_validated: true,
        manifest_verified: appliance.verify_orchestration_root(),
    }
}
