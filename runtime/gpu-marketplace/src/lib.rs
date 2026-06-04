//! GPU Marketplace v0.1 scaffold.
//!
//! The operational v0.1 model is deterministic shell logic in
//! `gpu/marketplace/marketplace_model.sh`. This Rust module mirrors the public
//! marketplace data contract so future runtime clients can bind to provider
//! identity, registration, assignment, artifact, verification, settlement
//! intent, reputation, replay, and lease-integration roots without giving GPU
//! providers authoritative state.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderIdentity {
    pub provider_id: String,
    pub node_id: String,
    pub registration_epoch: u64,
    pub capability_root: String,
    pub identity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationAction {
    Register,
    Update,
    Suspend,
    Recover,
    Retire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationEvent {
    pub provider_id: String,
    pub action: RegistrationAction,
    pub epoch: u64,
    pub registration_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAdvertisement {
    pub provider_id: String,
    pub gpu_model: String,
    pub memory_mb: u64,
    pub queue_capacity: u32,
    pub render_classes: Vec<String>,
    pub availability: String,
    pub capability_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapacityDeclaration {
    pub provider_id: String,
    pub available_slots: u32,
    pub reserved_slots: u32,
    pub consumed_slots: u32,
    pub epoch_capacity: u32,
    pub capacity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceAssignment {
    pub job_id: String,
    pub provider_id: String,
    pub assignment_id: String,
    pub assignment_epoch: u64,
    pub assignment_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSubmission {
    pub provider_id: String,
    pub job_id: String,
    pub artifact_id: String,
    pub submission_epoch: u64,
    pub artifact_submission_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationEvidence {
    pub artifact_id: String,
    pub artifact_integrity: bool,
    pub job_match: bool,
    pub provider_match: bool,
    pub projection_match: bool,
    pub verification_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementIntent {
    pub provider_id: String,
    pub work_completed: String,
    pub verification_result: String,
    pub reward_units: u64,
    pub settlement_intent_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReputation {
    pub provider_id: String,
    pub successful_jobs: u64,
    pub failed_jobs: u64,
    pub verified_artifacts: u64,
    pub provider_score: u64,
    pub reputation_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceReplay {
    pub marketplace_root: String,
    pub replay_root: String,
}

impl MarketplaceReplay {
    pub fn verifies(&self) -> bool {
        self.marketplace_root == self.replay_root
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseMarketplaceIntegration {
    pub projection_export_root: String,
    pub marketplace_submission_root: String,
    pub artifact_import_root: String,
    pub verification_import_root: String,
    pub lease_integration_root: String,
}
