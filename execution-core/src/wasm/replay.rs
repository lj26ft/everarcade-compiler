use super::checkpoints::ExecutionCheckpoint;
use super::execution::{execute_contract, ContractExecutionRequest};
use super::receipts::DeterministicExecutionReceipt;
use super::storage::HostOwnedState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayValidationReceipt {
    pub same_new_root: bool,
    pub same_receipt_hash: bool,
    pub same_mutation_hash: bool,
    pub same_stdout_hash: bool,
    pub same_fuel_used: bool,
}

pub fn replay_equivalence(
    module_bytes: &[u8],
    request: ContractExecutionRequest,
    previous_state: HostOwnedState,
    fuel_budget: u64,
    expected_receipt: &DeterministicExecutionReceipt,
    expected_checkpoint: &ExecutionCheckpoint,
) -> anyhow::Result<ReplayValidationReceipt> {
    let replay = execute_contract(module_bytes, request, previous_state, fuel_budget)?;
    Ok(ReplayValidationReceipt {
        same_new_root: replay.receipt.new_state_root == expected_checkpoint.new_state_root,
        same_receipt_hash: replay.receipt.receipt_hash()? == expected_receipt.receipt_hash()?,
        same_mutation_hash: replay.receipt.mutation_hash == expected_receipt.mutation_hash,
        same_stdout_hash: replay.receipt.stdout_hash == expected_receipt.stdout_hash,
        same_fuel_used: replay.receipt.fuel_used == expected_receipt.fuel_used,
    })
}
