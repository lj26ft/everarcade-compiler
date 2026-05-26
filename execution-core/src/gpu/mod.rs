use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuExecutionTask { pub task_id: String, pub tick: u64, pub input_hash: String, pub workload: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuWitness { pub task_id: String, pub output_hash: String, pub worker_id: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuExecutionBoundary { pub authoritative_tick: u64, pub allowed_workloads: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuResultEnvelope { pub task: GpuExecutionTask, pub witness: GpuWitness, pub replay_anchor: GpuReplayAnchor }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuReplayAnchor { pub task_id: String, pub authoritative_state_root: String, pub replay_hash: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuValidationRoot { pub tick: u64, pub validation_root: String }
