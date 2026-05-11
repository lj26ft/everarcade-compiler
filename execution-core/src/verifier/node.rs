use crate::{execute, hashing, ExecutionPlan, ExecutionReceipt, VmInput};

use super::replay::{ReplayEngine, ReplayResult};

#[derive(Debug, Clone)]
pub struct ContractWasm {
    pub contract_id: String,
    pub wasm_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct VerifierExecutionBundle {
    pub snapshot_state: crate::State,
    pub plan: ExecutionPlan,
    pub contracts: Vec<ContractWasm>,
    pub expected_receipt: Option<ExecutionReceipt>,
}

#[derive(Debug, Clone)]
pub struct VerifierResult {
    pub replay: ReplayResult,
    pub challenge_triggered: bool,
}

#[derive(Debug, Clone)]
pub struct VerifierNode {
    pub node_id: String,
}

impl VerifierNode {
    pub fn new(node_id: impl Into<String>) -> Self { Self { node_id: node_id.into() } }

    pub fn execute_locally(&self, input: VmInput) -> crate::VmOutput { execute::execute_vm(input) }

    pub fn verify_bundle(&self, bundle: &VerifierExecutionBundle) -> VerifierResult {
        let replay = ReplayEngine::replay(bundle);
        let challenge_triggered = bundle
            .expected_receipt
            .as_ref()
            .map(|remote| remote.receipt_hash != replay.receipt.receipt_hash)
            .unwrap_or(false);
        VerifierResult { replay, challenge_triggered }
    }

    pub fn contract_hashes(&self, contracts: &[ContractWasm]) -> Vec<String> {
        contracts.iter().map(|c| hashing::compute_contract_hash(&c.wasm_bytes)).collect()
    }
}
