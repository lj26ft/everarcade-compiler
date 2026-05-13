use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostExecutionOutput {
    pub abi_version: u64,
    pub vm_output_root: Hash,
    pub encoded_receipt: Vec<u8>,
}
