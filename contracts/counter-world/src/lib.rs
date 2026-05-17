use everarcade_abi::{ExecutionReceipt, StateChange, VmInput, VmOutput, ABI_VERSION};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_INPUT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const STATUS_OK: &str = "ok";
const STATUS_ERR: &str = "err";

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> u32 {
    if size == 0 || size as usize > MAX_OUTPUT_BYTES {
        return 0;
    }
    let mut buffer = Vec::<u8>::with_capacity(size as usize);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr as u32
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: u32, size: u32) {
    if ptr == 0 || size == 0 {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(ptr as *mut u8, 0, size as usize));
    }
}

#[no_mangle]
pub extern "C" fn everarcade_execute(input_ptr: u32, input_len: u32) -> u64 {
    if input_ptr == 0 || input_len == 0 || input_len as usize > MAX_INPUT_BYTES {
        return 0;
    }

    let input_bytes =
        unsafe { std::slice::from_raw_parts(input_ptr as *const u8, input_len as usize) };
    let output = match bincode::deserialize::<VmInput>(input_bytes) {
        Ok(input) => execute_counter(input),
        Err(_) => error_output(),
    };

    let output_bytes = match bincode::serialize(&output) {
        Ok(bytes) if !bytes.is_empty() && bytes.len() <= MAX_OUTPUT_BYTES => bytes,
        _ => return 0,
    };

    let out_ptr = alloc(output_bytes.len() as u32);
    if out_ptr == 0 {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            output_bytes.as_ptr(),
            out_ptr as *mut u8,
            output_bytes.len(),
        );
    }

    ((out_ptr as u64) << 32) | (output_bytes.len() as u64)
}

fn execute_counter(mut input: VmInput) -> VmOutput {
    let key = "counter".to_string();
    let before = input
        .state
        .get(&key)
        .cloned()
        .unwrap_or_else(|| "0".to_string());
    let next = before
        .parse::<u64>()
        .unwrap_or(0)
        .saturating_add(1)
        .to_string();
    input.state.insert(key.clone(), next.clone());

    let state_change = StateChange {
        key,
        before,
        after: next,
    };

    let mut node_hashes = BTreeMap::new();
    node_hashes.insert("counter-world".to_string(), hash_hex(b"counter-world"));

    let output_hash = hash_hex(&bincode::serialize(&input.state).unwrap_or_default());
    let mut receipt = ExecutionReceipt {
        protocol_epoch: input.protocol_epoch_id,
        abi_version: ABI_VERSION.to_string(),
        contract_hash: hash_hex(b"counter-world-v1"),
        input_hash: hash_hex(&bincode::serialize(&input).unwrap_or_default()),
        previous_state_root: hash_hex(STATUS_OK.as_bytes()),
        new_state_root: hash_hex(output_hash.as_bytes()),
        execution_root: hash_hex(b"counter-execution"),
        fuel_used: 1,
        memory_used: 0,
        node_hashes,
        state_changes: vec![state_change],
        output_hash: output_hash.clone(),
        receipt_hash: String::new(),
        snapshot_hash: hash_hex(b"counter-snapshot"),
        previous_snapshot_hash: None,
    };
    receipt.receipt_hash = hash_hex(
        format!(
            "{}:{}:{}",
            receipt.input_hash, receipt.output_hash, receipt.new_state_root
        )
        .as_bytes(),
    );

    VmOutput {
        updated_state: input.state,
        receipt,
    }
}

fn error_output() -> VmOutput {
    VmOutput {
        updated_state: BTreeMap::new(),
        receipt: ExecutionReceipt {
            protocol_epoch: 0,
            abi_version: ABI_VERSION.to_string(),
            contract_hash: hash_hex(STATUS_ERR.as_bytes()),
            input_hash: String::new(),
            previous_state_root: String::new(),
            new_state_root: String::new(),
            execution_root: String::new(),
            fuel_used: 0,
            memory_used: 0,
            node_hashes: BTreeMap::new(),
            state_changes: vec![],
            output_hash: String::new(),
            receipt_hash: String::new(),
            snapshot_hash: String::new(),
            previous_snapshot_hash: None,
        },
    }
}

fn hash_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
