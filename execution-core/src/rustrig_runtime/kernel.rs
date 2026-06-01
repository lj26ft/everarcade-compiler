use contract_api::protocol_records::RecordFields;
use contract_api::rustrig::{RustrigContext, RustrigOutput};
use serde::{Deserialize, Serialize};

use super::error::Result;
use super::executor::{stable_hash, RustrigExecutor};
use super::receipt::ExecutionReceipt;
use super::record_application::{AppliedRecord, AuthoritativeState, RecordApplication};
use super::registry::RustrigRegistry;
use super::replay::ReplayLog;
use super::validation::validate_output_abi;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub rustrig_id: String,
    pub version: String,
    pub context: RustrigContext,
    pub payload: RecordFields,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelExecution {
    pub output: RustrigOutput,
    pub applied: Vec<AppliedRecord>,
    pub receipt: ExecutionReceipt,
}

pub struct RustrigKernel {
    pub registry: RustrigRegistry,
    pub state: AuthoritativeState,
    pub replay: ReplayLog,
}

impl Default for RustrigKernel {
    fn default() -> Self {
        Self::new(RustrigRegistry::with_builtins())
    }
}
impl RustrigKernel {
    pub fn new(registry: RustrigRegistry) -> Self {
        Self {
            registry,
            state: AuthoritativeState::default(),
            replay: ReplayLog::default(),
        }
    }
    pub fn execute(&mut self, request: ExecutionRequest) -> Result<KernelExecution> {
        self.registry
            .validate_version(&request.rustrig_id, &request.version)?;
        let rig = self.registry.lookup(&request.rustrig_id)?.clone();
        let executor = RustrigExecutor::new(self.registry.clone());
        let output = executor.execute(&request.rustrig_id, &request.context, &request.payload)?;
        validate_output_abi(&rig, &output.records)?;
        let applied = RecordApplication::apply_all(&mut self.state, &output.records)?;
        let record_root = RustrigExecutor::record_root(&output.records);
        let state_root = stable_hash(&self.state);
        let replay_root = stable_hash(&self.replay.events);
        let checkpoint_root =
            stable_hash(&(request.context.checkpoint_root.clone(), state_root.clone()));
        let receipt = ExecutionReceipt {
            rustrig_id: output.rustrig_id.clone(),
            version: output.version.clone(),
            input_hash: request.context.input_hash,
            output_hash: output.output_hash.clone(),
            record_count: output.records.len(),
            record_root,
            state_root,
            replay_root,
            checkpoint_root,
        };
        self.replay.append_execution(&receipt);
        Ok(KernelExecution {
            output,
            applied,
            receipt,
        })
    }
}
