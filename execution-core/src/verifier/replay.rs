use crate::{execute, hashing, ExecutionReceipt, VmInput};

use super::node::VerifierExecutionBundle;

#[derive(Debug, Clone)]
pub struct ReplayResult {
    pub receipt: ExecutionReceipt,
    pub contract_hashes: Vec<String>,
}

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(bundle: &VerifierExecutionBundle) -> ReplayResult {
        let vm_input = VmInput {
            protocol_epoch_id: 1,
            state: bundle.snapshot_state.clone(),
            plan: bundle.plan.clone(),
        };
        let out = execute::execute_vm(vm_input);
        let contract_hashes = bundle
            .contracts
            .iter()
            .map(|contract| hashing::compute_contract_hash(&contract.wasm_bytes))
            .collect();

        ReplayResult {
            receipt: out.receipt,
            contract_hashes,
        }
    }
}
