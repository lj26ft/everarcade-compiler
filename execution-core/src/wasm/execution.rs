use crate::hashing::sha256;

use super::abi::{decode, decode_handle, encode, AbiMutationSet, CanonicalAbiEnvelope};
use super::checkpoints::ExecutionCheckpoint;
use super::engine::{DeterministicExecutionConfig, DeterministicWasmEngine};
use super::instance::instantiate;
use super::memory::{read_memory, write_memory};
use super::mutations::ExecutionMutationSet;
use super::receipts::DeterministicExecutionReceipt;
use super::serialization::canonical_bytes;
use super::state::StatefulExecutionRuntime;
use super::storage::HostOwnedState;
use wasmtime::{Linker, Module, Store};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContractExecutionRequest {
    pub contract_id: String,
    pub input: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExecutionStatus {
    Success,
    FuelExhausted,
    MalformedAbi,
    DuplicateMutation,
    Trap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeterministicExecutionResult {
    pub receipt: DeterministicExecutionReceipt,
    pub checkpoint: ExecutionCheckpoint,
    pub next_state: HostOwnedState,
}

pub fn execute_contract(
    module_bytes: &[u8],
    execution_request: ContractExecutionRequest,
    previous_state: HostOwnedState,
    fuel_budget: u64,
) -> anyhow::Result<DeterministicExecutionResult> {
    let det = DeterministicWasmEngine::new(DeterministicExecutionConfig::default())?;
    let loaded = det.compile_module(module_bytes)?;
    let mut store = Store::new(&det.engine, ());
    store.set_fuel(fuel_budget)?;

    let req_bytes = canonical_bytes(&execution_request)?;
    let req_hash = hex::encode(sha256(&req_bytes));
    let prev_root = state_root_host(&previous_state);

    let module = Module::from_binary(&det.engine, module_bytes)?;
    let linker = Linker::new(&det.engine);
    let instance = instantiate(&mut store, &linker, &module)?;
    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow::anyhow!("missing memory"))?;
    let alloc = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
    let execute = instance.get_typed_func::<(i32, i32), i64>(&mut store, "everarcade_execute")?;

    let req_envelope = CanonicalAbiEnvelope {
        version: 1,
        payload: req_bytes,
    };
    let env_bytes = encode(&req_envelope)?;
    let ptr = alloc.call(&mut store, i32::try_from(env_bytes.len())?)?;
    write_memory(&mut store, &memory, ptr, &env_bytes)?;

    let (mutations, stdout, mut status) =
        match execute.call(&mut store, (ptr, i32::try_from(env_bytes.len())?)) {
            Ok(raw) => {
                let (out_ptr, out_len) = decode_handle(raw as u64);
                if out_len == 0 {
                    (
                        ExecutionMutationSet { entries: vec![] },
                        vec![],
                        ExecutionStatus::MalformedAbi,
                    )
                } else {
                    let bytes = read_memory(&mut store, &memory, out_ptr as i32, out_len as i32)?;
                    let parsed = decode::<AbiMutationSet>(&bytes).or_else(|_| {
                        let s: anyhow::Result<String> = decode(&bytes);
                        s.and_then(|inner| decode::<AbiMutationSet>(inner.as_bytes()))
                    });
                    match parsed {
                        Ok(set) => {
                            let mutations = ExecutionMutationSet {
                                entries: set.mutations,
                            };
                            if StatefulExecutionRuntime::validate_mutations(&mutations).is_err() {
                                (
                                    ExecutionMutationSet { entries: vec![] },
                                    bytes,
                                    ExecutionStatus::DuplicateMutation,
                                )
                            } else {
                                (mutations, bytes, ExecutionStatus::Success)
                            }
                        }
                        Err(_) => (
                            ExecutionMutationSet { entries: vec![] },
                            bytes,
                            ExecutionStatus::MalformedAbi,
                        ),
                    }
                }
            }
            Err(trap) => {
                if trap.to_string().to_ascii_lowercase().contains("fuel") {
                    (
                        ExecutionMutationSet { entries: vec![] },
                        vec![],
                        ExecutionStatus::FuelExhausted,
                    )
                } else {
                    (
                        ExecutionMutationSet { entries: vec![] },
                        vec![],
                        ExecutionStatus::Trap,
                    )
                }
            }
        };

    let next_state = if status == ExecutionStatus::Success {
        previous_state.apply_mutations(&mutations)
    } else {
        previous_state.clone()
    };
    let new_root = state_root_host(&next_state);
    let mutation_hash = hex::encode(sha256(&canonical_bytes(&mutations.entries)?));
    let fuel_used = fuel_budget.saturating_sub(store.get_fuel().unwrap_or(fuel_budget));
    if status == ExecutionStatus::Trap && fuel_used >= fuel_budget {
        status = ExecutionStatus::FuelExhausted;
    }
    let stdout_hash = hex::encode(sha256(&stdout));

    let mut receipt = DeterministicExecutionReceipt {
        module_hash: loaded.module_hash,
        engine_config_hash: loaded.config_hash,
        abi_request_hash: req_hash,
        previous_state_root: prev_root,
        new_state_root: new_root,
        mutation_hash,
        fuel_budget,
        fuel_used,
        execution_status: status,
        stdout_hash,
        continuity_proof_hash: String::new(),
    };
    receipt.continuity_proof_hash = receipt.derive_continuity_proof_hash()?;
    let checkpoint = ExecutionCheckpoint::new(
        receipt.previous_state_root.clone(),
        receipt.new_state_root.clone(),
        receipt.receipt_hash()?,
        receipt.mutation_hash.clone(),
        receipt.module_hash.clone(),
    )?;

    Ok(DeterministicExecutionResult {
        receipt,
        checkpoint,
        next_state,
    })
}

fn state_root_host(state: &HostOwnedState) -> String {
    hex::encode(sha256(&canonical_bytes(&state.data).unwrap_or_default()))
}
