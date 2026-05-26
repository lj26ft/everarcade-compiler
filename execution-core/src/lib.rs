pub mod amendment;
pub mod api;
pub mod arbitration;
pub mod authority;
pub mod budget;
pub mod canonical;
pub mod capability;
pub mod checkpoint;
pub mod client_protocol;
pub mod codec;
pub mod compression;
pub mod consensus;
pub mod constitution;
pub mod continuity;
pub mod coordination;
pub mod dag;
pub mod deployment;
pub mod diagnostics;
pub mod divergence;
pub mod domain;
pub mod economics;
pub mod economy;
pub mod entity;
pub mod envelope;
pub mod epoch;
pub mod execute;
pub mod execution;
pub mod federation;
pub mod federation_runtime;
pub mod finality;
pub mod freeze;
pub mod genesis;
pub mod governance;
pub mod hash_runtime;
pub mod hashing;
pub mod interpretation;
pub mod jurisprudence;
pub mod leases;
pub mod lineage;
pub mod merkle;
pub mod migration;
pub mod namespace;
pub mod network;
pub mod operations;
pub mod operator;
pub mod package;
pub mod payload;
pub mod persistence;
pub mod proof;
pub mod protocol_upgrade;
pub mod pruning;
pub mod receipt_runtime;
pub mod reconciliation;
pub mod render_bridge;
pub mod replay;
pub mod replay_runtime;
pub mod runtime_commit;
pub mod runtime_semantics;
pub mod scheduler;
pub mod security;
pub mod session;
pub mod settlement;
pub mod simulation;
pub mod state;
pub mod state_diff;
pub mod state_engine;

pub mod state_root;

pub mod journal;
pub mod receipt_canonical;
pub mod sync;
pub mod tenancy;
pub mod topology;
pub mod trace;
pub mod treaty;
pub mod verifier;
pub mod wasm;
pub mod wasm_abi;
pub mod world;
pub mod world_partition;
pub mod world_scheduler;
pub mod xahau_gateway;

pub mod assets;
pub mod game_runtime;
pub mod gpu;
pub mod protocol;
pub mod xrpl;
pub use everarcade_abi::{
    ExecutionNode, ExecutionPlan, ExecutionReceipt, State, StateChange, VmInput, VmOutput,
    ABI_VERSION,
};

use std::sync::OnceLock;

static OUTPUT_BUFFER: OnceLock<Vec<u8>> = OnceLock::new();

fn set_output(bytes: Vec<u8>) {
    let _ = OUTPUT_BUFFER.set(bytes);
}

#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    if len <= 0 {
        return -1;
    }
    let mut buf = Vec::<u8>::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as i32
}

#[no_mangle]
pub extern "C" fn execute(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len < 0 {
        return -1;
    }
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let vm_input: VmInput = match everarcade_abi::deserialize(input_bytes) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let output = execute::execute_vm(vm_input);
    let bytes = match everarcade_abi::serialize(&output) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let out_ptr = bytes.as_ptr() as i32;
    set_output(bytes);
    out_ptr
}

#[no_mangle]
pub extern "C" fn output_len() -> i32 {
    OUTPUT_BUFFER.get().map(|b| b.len() as i32).unwrap_or(0)
}

pub mod civilization;
pub mod economic_asset;
pub mod external;
pub mod fiscal;
pub mod monetary;
pub mod treasury;

pub mod abi;
pub mod host;
pub mod identity;
pub mod vm;
pub mod zk;

pub mod xrpl_settlement;

pub mod dag_loader;

#[cfg(test)]
mod diagnostics_tests;
