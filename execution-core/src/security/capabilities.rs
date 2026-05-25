use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum RuntimeCapability {
    FilesystemAccess,
    StdoutEmission,
    EventEmission,
    SnapshotAccess,
    ReplayAccess,
    RestorationAccess,
    WitnessExtraction,
    CheckpointMutation,
    PartitionMerge,
    ValidationExport,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityScope {
    pub actor: String,
    pub execution_epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityToken {
    pub scope: CapabilityScope,
    pub granted: BTreeSet<RuntimeCapability>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityBoundary {
    pub implicit_privileges_disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityEnvelope {
    pub boundary: CapabilityBoundary,
    pub token: CapabilityToken,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityViolation {
    pub requested: RuntimeCapability,
    pub actor: String,
}

impl CapabilityEnvelope {
    pub fn enforce(&self, capability: RuntimeCapability) -> Result<(), CapabilityViolation> {
        if self.token.granted.contains(&capability) {
            Ok(())
        } else {
            Err(CapabilityViolation {
                requested: capability,
                actor: self.token.scope.actor.clone(),
            })
        }
    }
}
