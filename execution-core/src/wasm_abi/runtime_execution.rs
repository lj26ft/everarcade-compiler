use crate::runtime_commit::{commit_execution, CommitInput, CommitOutput, StateChange};

use super::{abi::AbiRequest, errors::WasmAbiError, execution::execute_contract, fuel::FuelLimit};

pub fn execute_and_commit(
    wasm: &[u8],
    request: &AbiRequest,
    fuel_limit: FuelLimit,
    execution_id: String,
    previous_entry_hash: [u8; 32],
    expected_sequence_number: u64,
) -> Result<CommitOutput, WasmAbiError> {
    let (response, fuel) = execute_contract(wasm, request, fuel_limit)?;

    let is_noop = !response.success && response.state_writes.is_empty();
    let mut writes = response.state_writes;
    writes.sort_by(|a, b| a.key.cmp(&b.key));
    for window in writes.windows(2) {
        if window[0].key == window[1].key {
            return Err(WasmAbiError::Runtime("duplicate state write key".into()));
        }
    }

    let changes = writes
        .into_iter()
        .map(|w| {
            let before = request
                .state_reads
                .iter()
                .find(|r| r.key == w.key)
                .map(|r| r.value.clone())
                .unwrap_or_default();
            StateChange {
                key: w.key,
                before,
                after: w.value,
            }
        })
        .collect::<Vec<_>>();

    commit_execution(CommitInput {
        contract_id: request.context.contract_id.clone(),
        execution_id,
        previous_state_root: request.context.previous_state_root,
        state_changes: changes,
        fuel_used: fuel.consumed.0,
        previous_entry_hash,
        expected_sequence_number,
        is_noop,
    })
    .map_err(|e| WasmAbiError::Runtime(e.to_string()))
}
