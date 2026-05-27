#![allow(dead_code)]

pub mod adversarial;
pub mod anchor;
pub mod archive;
pub mod branch;
pub mod cache;
pub mod compression;
pub mod continuity;
pub mod continuity_chain;
pub mod corruption;
pub mod era;
pub mod export;
pub mod federation;
pub mod hydration;
pub mod import;
pub mod index;
pub mod io;
pub mod materialization;
pub mod proof_verification;
pub mod provenance;
pub mod query;
pub mod restore;
pub mod runtime_validation;
pub mod storage;
pub mod timeline;
pub mod verification;
pub mod versioning;

// Production + ActiveIntegration replay surface exports (deterministic integration contract).
pub use adversarial::detect_corruption;
pub use anchor::{
    HistoricalReplayAnchor, HistoricalReplayAnchorRoot, HistoricalReplayAnchorWindow,
};
pub use archive::{CivilizationArchiveManifest, CivilizationArchiveRuntime};
pub use branch::{ReplayBranch, ReplayForkProof, ReplayForkVerification};
pub use compression::{ReplayCompressionNode, ReplayCompressionRoot, ReplayCompressionTree};
pub use corruption::{HistoricalCorruptionMatrix, HistoricalCorruptionScenario};
pub use era::{HistoricalReplayEra, HistoricalReplayEraManifest, HistoricalReplayEraWindow};
pub use federation::HistoricalReplayFederationWindow;
pub use index::HistoricalReplayIndex;
pub use provenance::{ReplayProvenanceManifest, ReplayProvenanceProof, ReplayProvenanceRoot};
pub use query::{HistoricalReplayQuery, HistoricalReplayQueryRuntime};
pub use restore::{
    HistoricalReplayHydrationRuntime, HistoricalReplayHydrationWindow,
    HistoricalReplayRestorationSession, HistoricalRestorationVerificationRuntime,
    ReplayBranchProofRuntime, ReplayCompressionTreeBuilder, ReplayCompressionTreeRuntime,
    ReplayForkMaterialization,
};
pub use runtime_validation::HistoricalRuntimeValidationEngine;
pub use storage::{HistoricalArtifactManifest, HistoricalArtifactRecord, HistoricalArtifactStore};
pub use timeline::HistoricalReplayTimeline;

// Scaffold / non-authoritative replay restoration surface.
pub use hydration::CivilizationObserverRuntime;

pub fn history_is_non_authoritative() -> bool {
    true
}
