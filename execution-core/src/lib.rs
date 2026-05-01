use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub mod execute;
pub mod hashing;
pub mod receipt;
pub mod scheduler;
pub mod state;
pub mod abi;

pub type State = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub action: String,
    pub payload: BTreeMap<String, String>, // 🔥 NO JSON
    pub deps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInput {
    pub state: State,
    pub plan: ExecutionPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub abi_version: String,
    pub previous_state_root: String,
    pub new_state_root: String,
    pub execution_root: String,
    pub receipt_hash: String,
    pub node_hashes: BTreeMap<String, String>,
    pub state_changes: Vec<StateChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmOutput {
    pub updated_state: State,
    pub receipt: ExecutionReceipt,
}

//
// ============================================================
// BINARY OUTPUT BUFFER
// ============================================================
//

static OUTPUT_BUFFER: OnceLock<Vec<u8>> = OnceLock::new();

fn set_output(bytes: Vec<u8>) {
    let _ = OUTPUT_BUFFER.set(bytes);
}

fn get_output_len() -> usize {
    OUTPUT_BUFFER.get().map(|b| b.len()).unwrap_or(0)
}

fn get_output_ptr() -> *const u8 {
    OUTPUT_BUFFER
        .get()
        .map(|b| b.as_ptr())
        .unwrap_or(std::ptr::null())
}

//
// ============================================================
// WASM EXPORTS (BINARY ABI)
// ============================================================
//

#[no_mangle]
pub extern "C" fn vm_alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn vm_dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        drop(Vec::from_raw_parts(ptr, size, size));
    }
}

#[no_mangle]
pub extern "C" fn vm_output_len() -> usize {
    get_output_len()
}

#[no_mangle]
pub extern "C" fn vm_execute_with_len(ptr: *mut u8, len: usize) -> *const u8 {
    use execute::execute_vm;

    let input_bytes = unsafe {
        std::slice::from_raw_parts(ptr, len)
    };

    // 🔥 BINARY DESERIALIZATION (NO JSON)
    let vm_input: VmInput =
        bincode::deserialize(input_bytes)
            .expect("invalid VmInput binary");

    let output = execute_vm(vm_input);

    let bytes =
        bincode::serialize(&output)
            .expect("invalid VmOutput binary");

    set_output(bytes);

    get_output_ptr()
}
