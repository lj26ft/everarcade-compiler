pub mod archival;
pub mod checkpoint;
pub mod civilization;
pub mod continuity;
pub mod continuity_runtime;
pub mod continuity_validation;
pub mod dag;
pub mod diagnostics;
pub mod economy;
pub mod epochs;
pub mod equivalence;
pub mod events;
pub mod evolution;
pub mod inventory;
pub mod lanes;
pub mod lifecycle;
pub mod materialization;
pub mod metrics;
pub mod persistence;
pub mod replay_compression;
pub mod reports;
pub mod restoration;
pub mod runtime;
pub mod scheduler;
pub mod simulation;
pub mod snapshots;
pub mod validation;

pub use archival::CivilizationArchive;
pub use checkpoint::{LifecycleCheckpoint, SchedulerCheckpoint, WorldCheckpoint};
pub use civilization::CivilizationEntity;
pub use continuity_runtime::{
    verify_entity_equivalence, verify_inventory_equivalence, verify_replay_equivalence,
    verify_restore_equivalence, verify_validation_root_chain, verify_world_continuity,
};

pub use continuity::{
    ContinuityCursor, ContinuityDivergence, ContinuitySegment, ContinuityWindow,
    WorldContinuityRoot, WorldEpochChain, WorldLineage, WorldRestorationProof,
};
pub use continuity_validation::{
    validate_checkpoint_lineage_integrity, validate_epoch_continuity_integrity,
    validate_event_continuity_integrity, validate_replay_continuity_integrity,
    validate_snapshot_continuity_integrity, validate_window_continuity_integrity,
    validate_witness_continuity_integrity,
};
pub use dag::{
    ExecutionDependency, ExecutionEdge, ExecutionGraph, ExecutionNode, ExecutionPartition,
    ExecutionPartitionId, ExecutionPhase, ExecutionShard, PartitionRoot,
};
pub use diagnostics::PersistentWorldDiagnostics;
pub use economy::{EconomicLedgerCheckpoint, EconomyMutation};
pub use epochs::{
    AggregatedCheckpointRoot, AggregatedMutationRoot, AggregatedReceiptRoot, AggregatedStdoutRoot,
    EpochCheckpoint, EpochContinuityProof, EpochExecutionSummary, EpochWitness, ExecutionEpoch,
    ExecutionEpochId, RollingEpochAnchor, RollingEpochWindow, WindowMaterializationBoundary,
};
pub use events::{
    EventChunk, EventReplayAnchor, EventRoot, EventSegment, EventStream, EventStreamCursor,
    EventWindow, ExecutionEvent, StreamingEventArchive,
};
pub use evolution::EvolutionStage;
pub use inventory::{AssetContinuityRecord, InventoryMutation};
pub use lanes::{
    ExecutionLaneScheduler, LaneCheckpointBoundary, LaneCommitPhase, LaneExecutionQueue,
    LaneMergePhase,
};
pub use lifecycle::EntityLifecycle;
pub use materialization::{
    AggregatedEventRoot, AggregatedWitnessRoot, CompressedEpochBundle, CompressedPartitionDelta,
    EpochMaterializationReceipt, EpochMaterializationSummary, EpochWitnessBundle, EventArchive,
    EventChunkManifest, ExecutionWitnessBundle, MaterializedEpoch, MaterializedEventStream,
    MaterializedExecution, MaterializedPartition, PartitionWitness, ReplayMaterializationWindow,
    ReplayRestorationArtifact, SnapshotAnchor, SnapshotManifest, WorldSnapshot,
};
pub use replay_compression::{
    CompressedEpochRange, ExecutionWitness, IncrementalReplayWindow, ReplayAnchor,
    ReplayCompressionWindow, ReplayCursor, ReplayDelta, ReplayMaterializationCursor,
    ReplayMergeBoundary, ReplaySnapshot, ReplayWitness, StreamingWitnessBundle, WitnessChunk,
    WitnessCursor, WitnessSegment,
};
pub use restoration::{
    IncrementalRestorationReceipt, PartialWorldRestoration, PartitionRestoration,
    RestorationManifest, SegmentRestoration,
};
pub use runtime::{
    IncrementalWorldRuntime, RuntimeCheckpointWindow, RuntimeCommitCursor, RuntimeExecutionCursor,
    WorldRuntimeTick, WorldRuntimeWindow,
};
pub use scheduler::{DeterministicTick, ScheduledOperation, WorldScheduler};
pub use simulation::{PersistentWorldState, WorldSimulation};
pub use snapshots::{
    DeterministicParallelExecutor, IncrementalSnapshot, ParallelMergeBarrier,
    ParallelReplayBoundary, ParallelWitnessBoundary, SnapshotDelta, SnapshotSegment,
    SnapshotSegmentManifest,
};

pub use equivalence::{
    assert_epoch_equivalence, assert_lane_equivalence, assert_replay_equivalence,
    assert_restoration_equivalence, assert_runtime_equivalence, assert_snapshot_equivalence,
    assert_witness_equivalence,
};
pub use metrics::{
    ContinuityMetrics, EpochMetrics, EventMetrics, ExecutionMetrics, LaneMetrics, ReplayMetrics,
    RestorationMetrics, RuntimeMetrics, SnapshotMetrics, WitnessMetrics,
};
pub use reports::{
    ContinuityValidationReport, EpochValidationReport, LaneValidationReport,
    ReplayValidationReport, RuntimeValidationReport, SnapshotValidationReport,
};
pub use validation::{
    runtime_validation_root, FederatedReplayAnchor, FederationValidationSurface,
    RuntimeContinuityRoot, RuntimeEquivalenceRoot, RuntimeValidationRoot, ValidationAnchor,
    ValidationArchive, ValidationArtifact, ValidationCursor, ValidationManifest,
    ValidationProofBundle, ValidationWindow,
};
