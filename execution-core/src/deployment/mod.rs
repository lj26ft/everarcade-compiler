pub mod audit;
pub mod authority;
pub mod bootstrap;
pub mod consensus_boundary;
pub mod continuity;
pub mod deterministic_io;
pub mod diagnostics;
pub mod distribution;
pub mod federation;
pub mod governance;
pub mod lifecycle;
pub mod manifests;
pub mod migration;
pub mod node;
pub mod observability;
pub mod operator;
pub mod orchestration;
pub mod packaging;
pub mod persistence;
pub mod provisioning;
pub mod quarantine;
pub mod recovery;
pub mod registry;
pub mod release;
pub mod reports;
pub mod restoration;
pub mod runtime_image;
pub mod scheduler;
pub mod synchronization;
pub mod telemetry;
pub mod topology;
pub mod validation;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeNodeIdentity {
    pub namespace: String,
    pub node_name: String,
    pub deterministic_id: String,
}

impl RuntimeNodeIdentity {
    pub fn new(namespace: impl Into<String>, node_name: impl Into<String>) -> Self {
        let namespace = namespace.into();
        let node_name = node_name.into();
        let deterministic_id = hash_hex(&(namespace.clone() + ":" + &node_name));
        Self {
            namespace,
            node_name,
            deterministic_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeNodeManifest {
    pub identity: RuntimeNodeIdentity,
    pub package_hash: String,
    pub release: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeNodeLifecycle {
    Genesis,
    Bootstrapped,
    Validated,
    Active,
    Synchronizing,
    Quarantined,
    Recovering,
    Archived,
    Restored,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifecycleReceipt {
    pub from: RuntimeNodeLifecycle,
    pub to: RuntimeNodeLifecycle,
    pub continuity_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeContinuityEnvelope {
    pub current: RuntimeNodeLifecycle,
    pub lineage: Vec<LifecycleReceipt>,
}

impl Default for NodeContinuityEnvelope {
    fn default() -> Self {
        Self {
            current: RuntimeNodeLifecycle::Genesis,
            lineage: vec![],
        }
    }
}

impl NodeContinuityEnvelope {
    pub fn transition(&mut self, to: RuntimeNodeLifecycle) -> Result<LifecycleReceipt, String> {
        if !valid_transition(self.current, to) {
            return Err(format!(
                "invalid transition: {:?} -> {:?}",
                self.current, to
            ));
        }
        let continuity_hash = hash_hex(&format!(
            "{:?}->{:?}:{}",
            self.current,
            to,
            self.lineage.len()
        ));
        let rec = LifecycleReceipt {
            from: self.current,
            to,
            continuity_hash,
        };
        self.current = to;
        self.lineage.push(rec.clone());
        Ok(rec)
    }

    pub fn continuity_root(&self) -> String {
        hash_hex(&bincode::serialize(&self.lineage).expect("lineage serializable"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SovereignRuntimeNode {
    pub manifest: RuntimeNodeManifest,
    pub continuity: NodeContinuityEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapManifest {
    pub genesis_hash: String,
    pub expected_package_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenesisRuntimePackage {
    pub name: String,
    pub bytes: Vec<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeBootstrapEnvelope {
    pub manifest: BootstrapManifest,
    pub package: GenesisRuntimePackage,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapVerificationReport {
    pub verified: bool,
    pub verification_hash: String,
}
impl RuntimeBootstrapEnvelope {
    pub fn verify(&self) -> BootstrapVerificationReport {
        let p = hash_hex(&self.package.bytes);
        let verified = p == self.manifest.expected_package_hash;
        BootstrapVerificationReport {
            verified,
            verification_hash: hash_hex(&format!("{}:{}", self.manifest.genesis_hash, p)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimePackage {
    pub name: String,
    pub payload: Vec<u8>,
}
impl RuntimePackage {
    pub fn package_hash(&self) -> String {
        hash_hex(&self.payload)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeImage {
    pub package: RuntimePackage,
    pub image_hash: String,
}
impl RuntimeImage {
    pub fn from_package(package: RuntimePackage) -> Self {
        let image_hash = hash_hex(&(package.name.clone() + ":" + &package.package_hash()));
        Self {
            package,
            image_hash,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactProof {
    pub artifact_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleManifest {
    pub artifact_hashes: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentBundle {
    pub artifacts: Vec<ArtifactProof>,
    pub manifest: BundleManifest,
}
impl DeploymentBundle {
    pub fn new(mut hashes: Vec<String>) -> Self {
        hashes.sort();
        let artifacts = hashes
            .iter()
            .cloned()
            .map(|h| ArtifactProof { artifact_hash: h })
            .collect();
        Self {
            artifacts,
            manifest: BundleManifest {
                artifact_hashes: hashes,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub version: String,
    pub image_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseLineage {
    pub versions: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeRelease {
    pub manifest: ReleaseManifest,
    pub lineage: ReleaseLineage,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeUpgradeEnvelope {
    pub from: String,
    pub to: String,
    pub upgrade_hash: String,
}
impl RuntimeUpgradeEnvelope {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        let (from, to) = (from.into(), to.into());
        Self {
            upgrade_hash: hash_hex(&(from.clone() + "->" + &to)),
            from,
            to,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityBoundary {
    pub min_version: String,
    pub max_version: String,
}
impl CompatibilityBoundary {
    pub fn supports(&self, version: &str) -> bool {
        version >= self.min_version.as_str() && version <= self.max_version.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationNodeRecord {
    pub node_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationTopology {
    pub members: Vec<FederationNodeRecord>,
}
impl FederationTopology {
    pub fn canonical(mut ids: Vec<String>) -> Self {
        ids.sort();
        Self {
            members: ids
                .into_iter()
                .map(|node_id| FederationNodeRecord { node_id })
                .collect(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SynchronizationWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationContinuityEnvelope {
    pub topology_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartitionBoundary {
    pub partition_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryCheckpoint {
    pub checkpoint_hash: String,
    pub height: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryPlan {
    pub checkpoints: Vec<RecoveryCheckpoint>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryBoundary {
    pub boundary_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestorationEnvelope {
    pub plan: RecoveryPlan,
    pub boundary: RecoveryBoundary,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuityRecoveryReport {
    pub recovered: bool,
    pub report_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentQuarantine {
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IsolationRecoveryPlan {
    pub steps: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorruptionBoundary {
    pub boundary_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalContainment {
    pub containment_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalLedger {
    pub entries: Vec<String>,
}
impl OperationalLedger {
    pub fn append(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }
    pub fn root(&self) -> String {
        hash_hex(&bincode::serialize(&self.entries).expect("entries serializable"))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentCheckpoint {
    pub root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistenceBoundary {
    pub boundary_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalArchive {
    pub snapshots: Vec<OperationalSnapshot>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalSnapshot {
    pub snapshot_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeTelemetryEnvelope {
    pub event_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterministicDiagnosticReport {
    pub report_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalAuditRecord {
    pub audit_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeHealthEnvelope {
    pub health_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceManifest {
    pub policy_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatorAuthority {
    pub operator_id: String,
    pub predecessor: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalPolicy {
    pub policy_id: String,
    pub revision: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeDirective {
    pub directive_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceTransition {
    pub from_policy: String,
    pub to_policy: String,
    pub transition_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalScheduler {
    pub queue: Vec<DeterministicTaskEnvelope>,
}
impl OperationalScheduler {
    pub fn ordered(mut queue: Vec<DeterministicTaskEnvelope>) -> Self {
        queue.sort_by(|a, b| a.task_id.cmp(&b.task_id));
        Self { queue }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeploymentTick {
    pub tick: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeterministicTaskEnvelope {
    pub task_id: String,
    pub tick: DeploymentTick,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerContinuity {
    pub scheduler_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsensusBoundary {
    pub boundary_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalSettlementAnchor {
    pub anchor_id: String,
    pub anchor_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XRPLAnchorBoundary {
    pub network: String,
    pub anchor: ExternalSettlementAnchor,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationSettlementEnvelope {
    pub federation_hash: String,
    pub anchor_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributionManifest {
    pub channel: DeploymentChannel,
    pub artifacts: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentChannel {
    Stable,
    Candidate,
    Emergency,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseDistributionEnvelope {
    pub manifest_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactReplicationBoundary {
    pub replication_hash: String,
}

pub fn hash_hex(data: impl AsRef<[u8]>) -> String {
    let mut h = Sha256::new();
    h.update(data.as_ref());
    format!("{:x}", h.finalize())
}

fn valid_transition(from: RuntimeNodeLifecycle, to: RuntimeNodeLifecycle) -> bool {
    matches!(
        (from, to),
        (
            RuntimeNodeLifecycle::Genesis,
            RuntimeNodeLifecycle::Bootstrapped
        ) | (
            RuntimeNodeLifecycle::Bootstrapped,
            RuntimeNodeLifecycle::Validated
        ) | (
            RuntimeNodeLifecycle::Validated,
            RuntimeNodeLifecycle::Active
        ) | (
            RuntimeNodeLifecycle::Active,
            RuntimeNodeLifecycle::Synchronizing
        ) | (
            RuntimeNodeLifecycle::Synchronizing,
            RuntimeNodeLifecycle::Active
        ) | (
            RuntimeNodeLifecycle::Active,
            RuntimeNodeLifecycle::Quarantined
        ) | (
            RuntimeNodeLifecycle::Quarantined,
            RuntimeNodeLifecycle::Recovering
        ) | (
            RuntimeNodeLifecycle::Recovering,
            RuntimeNodeLifecycle::Restored
        ) | (RuntimeNodeLifecycle::Restored, RuntimeNodeLifecycle::Active)
            | (_, RuntimeNodeLifecycle::Archived)
            | (
                RuntimeNodeLifecycle::Archived,
                RuntimeNodeLifecycle::Restored
            )
            | (_, RuntimeNodeLifecycle::Retired)
    )
}
