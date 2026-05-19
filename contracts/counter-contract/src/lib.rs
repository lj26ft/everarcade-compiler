use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AbiStateRead {
    key: Vec<u8>,
    value: Vec<u8>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AbiStateWrite {
    key: Vec<u8>,
    value: Vec<u8>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AbiExecutionContext {
    abi_version: u32,
    contract_id: String,
    contract_version: String,
    previous_state_root: [u8; 32],
    continuity_hash: [u8; 32],
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AbiRequest {
    context: AbiExecutionContext,
    input: Vec<u8>,
    state_reads: Vec<AbiStateRead>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AbiResponse {
    success: bool,
    output: Vec<u8>,
    state_writes: Vec<AbiStateWrite>,
    events: BTreeMap<Vec<u8>, Vec<u8>>,
}

#[no_mangle]
pub extern "C" fn alloc(len: u32) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len as usize);
    let p = buf.as_mut_ptr();
    std::mem::forget(buf);
    p
}

#[no_mangle]
pub extern "C" fn everarcade_execute(ptr: u32, len: u32) -> u64 {
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let req: AbiRequest = match bincode::deserialize(input) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let current = req
        .state_reads
        .iter()
        .find(|r| r.key == b"counter")
        .and_then(|r| std::str::from_utf8(&r.value).ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let next = current + 1;
    let resp = AbiResponse {
        success: true,
        output: next.to_string().into_bytes(),
        state_writes: vec![AbiStateWrite {
            key: b"counter".to_vec(),
            value: next.to_string().into_bytes(),
        }],
        events: BTreeMap::new(),
    };
    let encoded = bincode::serialize(&resp).unwrap_or_default();
    let out_ptr = alloc(encoded.len() as u32);
    unsafe { std::ptr::copy_nonoverlapping(encoded.as_ptr(), out_ptr, encoded.len()) };
    ((encoded.len() as u64) << 32) | (out_ptr as u64 & 0xffff_ffff)
}
