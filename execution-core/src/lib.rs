// FILE: execution-core/src/lib.rs
//
// PURPOSE:
// Deterministic protocol VM entrypoint compiled to WASM.
//
// THIS FILE ESTABLISHES:
// - stable VM boundary
// - host ↔ VM serialization contract
// - deterministic execution interface
// - pure execution pipeline
//
// DESIGN RULES:
// - NO filesystem
// - NO clocks
// - NO randomness
// - NO networking
// - NO host state mutation
//
// VM MUST REMAIN PURE.
//
// TARGET:
// wasm32-unknown-unknown

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod execute;
pub mod hashing;
pub mod receipt;
pub mod scheduler;
pub mod state;

pub type State = BTreeMap<String, String>;

//
// ============================================================
// EXECUTION NODE
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub action: String,
    pub payload: serde_json::Value,
    pub deps: Vec<String>,
}

//
// ============================================================
// EXECUTION PLAN
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}

//
// ============================================================
// VM INPUT
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInput {
    pub state: State,
    pub plan: ExecutionPlan,
}

//
// ============================================================
// STATE CHANGE
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}

//
// ============================================================
// RECEIPT
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub previous_state_root: String,
    pub new_state_root: String,
    pub execution_root: String,
    pub receipt_hash: String,
    pub node_hashes: BTreeMap<String, String>,
    pub state_changes: Vec<StateChange>,
}

//
// ============================================================
// VM OUTPUT
// ============================================================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmOutput {
    pub updated_state: State,
    pub receipt: ExecutionReceipt,
}

//
// ============================================================
// WASM ENTRYPOINT
// ============================================================
//
// HOST FLOW:
//
// 1. host serializes VmInput -> JSON
// 2. host writes bytes into WASM memory
// 3. host calls vm_execute(ptr, len)
// 4. VM executes deterministically
// 5. VM returns ptr to serialized VmOutput
//
// MEMORY MANAGEMENT:
// Host owns input memory.
// VM allocates output memory.
//
// ============================================================
//

#[no_mangle]
pub extern "C" fn vm_alloc(size: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(size);

    let ptr = buffer.as_mut_ptr();

    std::mem::forget(buffer);

    ptr
}

#[no_mangle]
pub extern "C" fn vm_dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        drop(Vec::from_raw_parts(ptr, 0, size));
    }
}

//
// ============================================================
// MAIN VM EXECUTION ENTRYPOINT
// ============================================================
//

#[no_mangle]
pub extern "C" fn vm_execute(ptr: *mut u8, len: usize) -> *mut u8 {
    //
    // READ INPUT BUFFER
    //

    let input_slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    //
    // DESERIALIZE INPUT
    //

    let input: VmInput =
        serde_json::from_slice(input_slice)
            .expect("failed to deserialize VmInput");

    //
    // EXECUTE DETERMINISTIC VM
    //

    let output = execute::execute_vm(input);

    //
    // SERIALIZE OUTPUT
    //

    let output_bytes =
        serde_json::to_vec(&output)
            .expect("failed to serialize VmOutput");

    //
    // RETURN OWNED BUFFER
    //

    let boxed = output_bytes.into_boxed_slice();

    Box::into_raw(boxed) as *mut u8
}

//
// ============================================================
// OUTPUT LENGTH HELPER
// ============================================================
//
// Host needs output length.
// Simplest temporary strategy:
// prepend u32 length later.
//
// For now:
// host reads exported OUTPUT_LEN global.
//
// ============================================================
//

static mut OUTPUT_LEN: usize = 0;

#[no_mangle]
pub extern "C" fn vm_execute_with_len(
    ptr: *mut u8,
    len: usize,
) -> *mut u8 {
    let input_slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    let input: VmInput =
        serde_json::from_slice(input_slice)
            .expect("failed to deserialize VmInput");

    let output = execute::execute_vm(input);

    let output_bytes =
        serde_json::to_vec(&output)
            .expect("failed to serialize VmOutput");

    unsafe {
        OUTPUT_LEN = output_bytes.len();
    }

    let boxed = output_bytes.into_boxed_slice();

    Box::into_raw(boxed) as *mut u8
}

#[no_mangle]
pub extern "C" fn vm_output_len() -> usize {
    unsafe { OUTPUT_LEN }
}
