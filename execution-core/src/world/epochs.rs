use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

use super::{checkpoint::WorldCheckpoint, events::EventStream};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionEpochId(pub u64);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedReceiptRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedMutationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedStdoutRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedCheckpointRoot(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochExecutionSummary {
    pub execution_id: String,
    pub receipt_hash: String,
    pub mutation_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochCheckpoint {
    pub epoch_id: ExecutionEpochId,
    pub checkpoint_root: AggregatedCheckpointRoot,
    pub world_checkpoint: WorldCheckpoint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochContinuityProof {
    pub previous_epoch_hash: String,
    pub epoch_hash: String,
    pub continuity_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEpoch {
    pub epoch_id: ExecutionEpochId,
    pub summaries: Vec<EpochExecutionSummary>,
    pub event_stream: EventStream,
    pub checkpoint: EpochCheckpoint,
}

impl ExecutionEpoch {
    pub fn aggregated_receipt_root(&self) -> Result<AggregatedReceiptRoot, String> {
        Ok(AggregatedReceiptRoot(hash_bytes(
            &canonical_encode(
                &self
                    .summaries
                    .iter()
                    .map(|s| (&s.execution_id, &s.receipt_hash))
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?,
        )))
    }
    pub fn aggregated_mutation_root(&self) -> Result<AggregatedMutationRoot, String> {
        Ok(AggregatedMutationRoot(hash_bytes(
            &canonical_encode(
                &self
                    .summaries
                    .iter()
                    .map(|s| (&s.execution_id, &s.mutation_hash))
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?,
        )))
    }
    pub fn aggregated_stdout_root(&self) -> Result<AggregatedStdoutRoot, String> {
        Ok(AggregatedStdoutRoot(self.event_stream.root()?.0))
    }
    pub fn epoch_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
    pub fn continuity_proof(
        &self,
        previous_epoch_hash: &str,
    ) -> Result<EpochContinuityProof, String> {
        let epoch_hash = self.epoch_hash()?;
        let continuity_hash = hash_bytes(
            &canonical_encode(&(
                previous_epoch_hash,
                &epoch_hash,
                &self.checkpoint.checkpoint_root.0,
            ))
            .map_err(|e| e.to_string())?,
        );
        Ok(EpochContinuityProof {
            previous_epoch_hash: previous_epoch_hash.into(),
            epoch_hash,
            continuity_hash,
        })
    }
    pub fn from_parts(
        epoch_id: u64,
        summaries: Vec<EpochExecutionSummary>,
        mut event_stream: EventStream,
        world_checkpoint: WorldCheckpoint,
    ) -> Result<Self, String> {
        event_stream.canonicalize();
        let checkpoint_root = AggregatedCheckpointRoot(hash_bytes(
            &canonical_encode(&world_checkpoint).map_err(|e| e.to_string())?,
        ));
        Ok(Self {
            epoch_id: ExecutionEpochId(epoch_id),
            summaries,
            event_stream,
            checkpoint: EpochCheckpoint {
                epoch_id: ExecutionEpochId(epoch_id),
                checkpoint_root,
                world_checkpoint,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochWitness {
    pub epoch_hash: String,
    pub receipt_root: AggregatedReceiptRoot,
    pub mutation_root: AggregatedMutationRoot,
    pub stdout_root: AggregatedStdoutRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollingEpochWindow {
    pub start_epoch: u64,
    pub end_epoch: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollingEpochAnchor {
    pub epoch: u64,
    pub epoch_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowMaterializationBoundary {
    pub trim_before_epoch: u64,
}
