use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{
    checkpoint::WorldCheckpoint,
    dag::{ExecutionGraph, ExecutionPartition, ExecutionPartitionId},
    epochs::{
        AggregatedCheckpointRoot, AggregatedMutationRoot, AggregatedReceiptRoot, ExecutionEpochId,
    },
    events::{EventChunk, EventStream, ExecutionEvent},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedEventRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregatedWitnessRoot(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedExecution {
    pub execution_id: String,
    pub partition_id: String,
    pub ordinal: u64,
    pub receipt_hash: String,
    pub mutation_hash: String,
    pub event_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedPartition {
    pub partition_id: ExecutionPartitionId,
    pub executions: Vec<MaterializedExecution>,
    pub mutation_root: String,
    pub receipt_root: String,
    pub event_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochWitnessBundle {
    pub epoch_id: ExecutionEpochId,
    pub partition_witnesses: Vec<PartitionWitness>,
    pub aggregated_witness_root: AggregatedWitnessRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionWitness {
    pub partition_id: ExecutionPartitionId,
    pub execution_witnesses: Vec<ExecutionWitnessBundle>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionWitnessBundle {
    pub execution_id: String,
    pub receipt_hash: String,
    pub mutation_hash: String,
    pub event_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedEpoch {
    pub epoch_id: ExecutionEpochId,
    pub partitions: Vec<MaterializedPartition>,
    pub event_stream: MaterializedEventStream,
    pub receipt_root: AggregatedReceiptRoot,
    pub mutation_root: AggregatedMutationRoot,
    pub event_root: AggregatedEventRoot,
    pub checkpoint_root: AggregatedCheckpointRoot,
    pub witness_root: AggregatedWitnessRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochMaterializationReceipt {
    pub epoch_hash: String,
    pub checkpoint_root: AggregatedCheckpointRoot,
    pub continuity_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochMaterializationSummary {
    pub partition_count: usize,
    pub execution_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressedPartitionDelta {
    pub partition_id: ExecutionPartitionId,
    pub mutation_root: String,
    pub receipt_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressedEpochBundle {
    pub epoch_id: ExecutionEpochId,
    pub deltas: Vec<CompressedPartitionDelta>,
    pub event_root: AggregatedEventRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMaterializationWindow {
    pub start_epoch: ExecutionEpochId,
    pub end_epoch: ExecutionEpochId,
    pub bundles: Vec<CompressedEpochBundle>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRestorationArtifact {
    pub restored_epoch_hash: String,
    pub restored_checkpoint_root: AggregatedCheckpointRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventChunkManifest {
    pub chunk_index: u64,
    pub event_count: usize,
    pub chunk_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventArchive {
    pub chunks: Vec<EventChunkManifest>,
    pub root: AggregatedEventRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedEventStream {
    pub stream: EventStream,
    pub archive: EventArchive,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotAnchor {
    pub epoch_hash: String,
    pub checkpoint_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotManifest {
    pub world_id: String,
    pub epoch_id: ExecutionEpochId,
    pub event_root: AggregatedEventRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub manifest: SnapshotManifest,
    pub anchor: SnapshotAnchor,
    pub checkpoint: WorldCheckpoint,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestorationReceipt {
    pub snapshot_hash: String,
    pub restoration_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLane(pub u32);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneAssignment {
    pub partition_id: ExecutionPartitionId,
    pub lane: ExecutionLane,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicParallelBoundary {
    pub assignments: Vec<LaneAssignment>,
}

pub struct EpochMaterializer;
impl EpochMaterializer {
    pub fn commit_epoch(
        epoch_id: u64,
        graph: &ExecutionGraph,
        partitions: &[ExecutionPartition],
        checkpoint: WorldCheckpoint,
        previous_epoch_hash: &str,
    ) -> Result<
        (
            MaterializedEpoch,
            EpochMaterializationReceipt,
            EpochWitnessBundle,
            ReplayRestorationArtifact,
        ),
        String,
    > {
        let canonical_nodes = graph.canonical_order()?;
        let mut node_to_partition: BTreeMap<String, ExecutionPartitionId> = BTreeMap::new();
        for p in partitions {
            for id in &p.node_ids {
                node_to_partition.insert(id.clone(), p.id.clone());
            }
        }
        let mut per_partition: BTreeMap<ExecutionPartitionId, Vec<MaterializedExecution>> =
            BTreeMap::new();
        let mut all_events = vec![];
        for (i, node_id) in canonical_nodes.iter().enumerate() {
            let pid = node_to_partition
                .get(node_id)
                .ok_or_else(|| format!("node missing partition: {node_id}"))?;
            let receipt_hash =
                hash_bytes(&canonical_encode(&("receipt", node_id)).map_err(|e| e.to_string())?);
            let mutation_hash =
                hash_bytes(&canonical_encode(&("mutation", node_id)).map_err(|e| e.to_string())?);
            let event_hash =
                hash_bytes(&canonical_encode(&("event", node_id)).map_err(|e| e.to_string())?);
            let exec = MaterializedExecution {
                execution_id: node_id.clone(),
                partition_id: pid.0.clone(),
                ordinal: i as u64,
                receipt_hash: receipt_hash.clone(),
                mutation_hash: mutation_hash.clone(),
                event_hash: event_hash.clone(),
            };
            per_partition
                .entry(pid.clone())
                .or_default()
                .push(exec.clone());
            all_events.push(ExecutionEvent {
                execution_id: node_id.clone(),
                partition_id: pid.0.clone(),
                sequence: i as u64,
                payload: event_hash.into_bytes(),
            });
        }
        let mut mat_parts = vec![];
        for (pid, mut execs) in per_partition {
            execs.sort_by(|a, b| (a.ordinal, &a.execution_id).cmp(&(b.ordinal, &b.execution_id)));
            let receipt_root = hash_bytes(
                &canonical_encode(
                    &execs
                        .iter()
                        .map(|e| (&e.execution_id, &e.receipt_hash))
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| e.to_string())?,
            );
            let mutation_root = hash_bytes(
                &canonical_encode(
                    &execs
                        .iter()
                        .map(|e| (&e.execution_id, &e.mutation_hash))
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| e.to_string())?,
            );
            let event_root = hash_bytes(
                &canonical_encode(
                    &execs
                        .iter()
                        .map(|e| (&e.execution_id, &e.event_hash))
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| e.to_string())?,
            );
            let checkpoint_root = hash_bytes(
                &canonical_encode(&(pid.0.clone(), &receipt_root, &mutation_root, &event_root))
                    .map_err(|e| e.to_string())?,
            );
            mat_parts.push(MaterializedPartition {
                partition_id: pid,
                executions: execs,
                mutation_root,
                receipt_root,
                event_root,
                checkpoint_root,
            });
        }
        mat_parts.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));
        let mut stream = EventStream {
            chunks: vec![EventChunk {
                chunk_index: 0,
                events: all_events,
            }],
        };
        stream.canonicalize();
        let event_root = AggregatedEventRoot(stream.root()?.0);
        let archive = EventArchive {
            chunks: vec![EventChunkManifest {
                chunk_index: 0,
                event_count: stream.chunks[0].events.len(),
                chunk_hash: hash_bytes(
                    &canonical_encode(&stream.chunks[0]).map_err(|e| e.to_string())?,
                ),
            }],
            root: event_root.clone(),
        };
        let event_stream = MaterializedEventStream { stream, archive };
        let receipt_root = AggregatedReceiptRoot(hash_bytes(
            &canonical_encode(
                &mat_parts
                    .iter()
                    .map(|p| (&p.partition_id.0, &p.receipt_root))
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?,
        ));
        let mutation_root = AggregatedMutationRoot(hash_bytes(
            &canonical_encode(
                &mat_parts
                    .iter()
                    .map(|p| (&p.partition_id.0, &p.mutation_root))
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?,
        ));
        let checkpoint_root = AggregatedCheckpointRoot(hash_bytes(
            &canonical_encode(
                &mat_parts
                    .iter()
                    .map(|p| (&p.partition_id.0, &p.checkpoint_root))
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?,
        ));
        let witness = EpochWitnessBundle {
            epoch_id: ExecutionEpochId(epoch_id),
            partition_witnesses: mat_parts
                .iter()
                .map(|p| PartitionWitness {
                    partition_id: p.partition_id.clone(),
                    execution_witnesses: p
                        .executions
                        .iter()
                        .map(|e| ExecutionWitnessBundle {
                            execution_id: e.execution_id.clone(),
                            receipt_hash: e.receipt_hash.clone(),
                            mutation_hash: e.mutation_hash.clone(),
                            event_hash: e.event_hash.clone(),
                        })
                        .collect(),
                })
                .collect(),
            aggregated_witness_root: AggregatedWitnessRoot(hash_bytes(
                &canonical_encode(
                    &mat_parts
                        .iter()
                        .map(|p| {
                            (
                                &p.partition_id.0,
                                &p.receipt_root,
                                &p.mutation_root,
                                &p.event_root,
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| e.to_string())?,
            )),
        };
        let epoch = MaterializedEpoch {
            epoch_id: ExecutionEpochId(epoch_id),
            partitions: mat_parts,
            event_stream,
            receipt_root,
            mutation_root,
            event_root,
            checkpoint_root: checkpoint_root.clone(),
            witness_root: witness.aggregated_witness_root.clone(),
        };
        let epoch_hash = hash_bytes(&canonical_encode(&epoch).map_err(|e| e.to_string())?);
        let continuity_hash = hash_bytes(
            &canonical_encode(&(previous_epoch_hash, &epoch_hash, &checkpoint))
                .map_err(|e| e.to_string())?,
        );
        let receipt = EpochMaterializationReceipt {
            epoch_hash: epoch_hash.clone(),
            checkpoint_root: checkpoint_root.clone(),
            continuity_hash,
        };
        let artifact = ReplayRestorationArtifact {
            restored_epoch_hash: epoch_hash,
            restored_checkpoint_root: checkpoint_root,
        };
        Ok((epoch, receipt, witness, artifact))
    }

    pub fn replay_epoch(
        epoch_id: u64,
        graph: &ExecutionGraph,
        partitions: &[ExecutionPartition],
        checkpoint: WorldCheckpoint,
        previous_epoch_hash: &str,
    ) -> Result<MaterializedEpoch, String> {
        Self::commit_epoch(epoch_id, graph, partitions, checkpoint, previous_epoch_hash)
            .map(|v| v.0)
    }

    pub fn restore_epoch(epoch: &MaterializedEpoch) -> Result<ReplayRestorationArtifact, String> {
        let restored_epoch_hash = hash_bytes(&canonical_encode(epoch).map_err(|e| e.to_string())?);
        Ok(ReplayRestorationArtifact {
            restored_epoch_hash,
            restored_checkpoint_root: epoch.checkpoint_root.clone(),
        })
    }
}
