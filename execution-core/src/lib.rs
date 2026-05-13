pub mod execute;
pub mod hashing;
pub mod wasm;
pub mod state_engine;
pub mod verifier;
pub mod freeze;
pub mod protocol_upgrade;
pub mod settlement;
pub mod package;
pub mod network;
pub mod finality;
pub mod proof;
pub mod trace;
pub mod runtime_semantics;
pub mod economy;
pub mod entity;
pub mod federation;
pub mod execution;
pub mod receipt_runtime;
pub mod replay_runtime;
pub mod codec;
pub mod hash_runtime;
pub mod merkle;
pub mod checkpoint;
pub mod replay;
pub mod sync;
pub mod simulation;
pub mod pruning;
pub mod session;
pub mod compression;
pub mod epoch;
pub mod budget;
pub mod economics;
pub mod namespace;
pub mod tenancy;
pub mod capability;
pub mod domain;
pub mod migration;
pub mod arbitration;
pub mod treaty;
pub mod interpretation;
pub mod jurisprudence;
pub mod amendment;
pub mod constitution;

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
    if len <= 0 { return -1; }
    let mut buf = Vec::<u8>::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as i32
}

#[no_mangle]
pub extern "C" fn execute(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len < 0 { return -1; }
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let vm_input: VmInput = match everarcade_abi::deserialize(input_bytes) { Ok(v)=>v, Err(_)=> return -1 };
    let output = execute::execute_vm(vm_input);
    let bytes = match everarcade_abi::serialize(&output) { Ok(v)=>v, Err(_)=> return -1 };
    let out_ptr = bytes.as_ptr() as i32;
    set_output(bytes);
    out_ptr
}

#[no_mangle]
pub extern "C" fn output_len() -> i32 {
    OUTPUT_BUFFER.get().map(|b| b.len() as i32).unwrap_or(0)
}
