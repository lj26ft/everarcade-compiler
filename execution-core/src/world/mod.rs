pub mod archival;
pub mod checkpoint;
pub mod civilization;
pub mod continuity;
pub mod dag;
pub mod diagnostics;
pub mod economy;
pub mod epochs;
pub mod events;
pub mod evolution;
pub mod inventory;
pub mod lifecycle;
pub mod materialization;
pub mod persistence;
pub mod replay_compression;
pub mod restoration;
pub mod scheduler;
pub mod simulation;

pub use archival::CivilizationArchive;
pub use checkpoint::{LifecycleCheckpoint, SchedulerCheckpoint, WorldCheckpoint};
pub use civilization::CivilizationEntity;
pub use continuity::{
    ContinuityDivergence, WorldContinuityRoot, WorldEpochChain, WorldLineage, WorldRestorationProof,
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
    ExecutionEpochId,
};
pub use events::{EventChunk, EventRoot, EventStream, ExecutionEvent};
pub use evolution::EvolutionStage;
pub use inventory::{AssetContinuityRecord, InventoryMutation};
pub use lifecycle::EntityLifecycle;
pub use materialization::*;
pub use replay_compression::{
    CompressedEpochRange, ExecutionWitness, ReplayAnchor, ReplayCompressionWindow, ReplaySnapshot,
    ReplayWitness,
};
pub use restoration::RestorationManifest;
pub use scheduler::{DeterministicTick, ScheduledOperation, WorldScheduler};
pub use simulation::{PersistentWorldState, WorldSimulation};
