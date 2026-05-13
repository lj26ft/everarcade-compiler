use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostExecutionInput {
    pub abi_version: u64,
    pub vm_input_root: Hash,
    pub encoded_package: Vec<u8>,
}
