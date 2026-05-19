use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const EVERARCADE_ABI_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiExecutionContext {
    pub abi_version: u32,
    pub contract_id: String,
    pub contract_version: String,
    pub previous_state_root: [u8; 32],
    pub continuity_hash: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiStateRead {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiStateWrite {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiRequest {
    pub context: AbiExecutionContext,
    pub input: Vec<u8>,
    pub state_reads: Vec<AbiStateRead>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiFuelReport {
    pub fuel_limit: u64,
    pub fuel_used: u64,
    pub exhausted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiReceipt {
    pub contract_hash: [u8; 32],
    pub abi_version: u32,
    pub fuel_used: u64,
    pub execution_hash: [u8; 32],
    pub previous_state_root: [u8; 32],
    pub new_state_root: [u8; 32],
    pub state_diff_hash: [u8; 32],
    pub continuity_hash: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbiResponse {
    pub success: bool,
    pub output: Vec<u8>,
    pub state_writes: Vec<AbiStateWrite>,
    pub events: BTreeMap<Vec<u8>, Vec<u8>>,
    pub receipt: AbiReceipt,
    pub fuel: AbiFuelReport,
}
