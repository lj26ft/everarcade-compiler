use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostExecutionReceipt {
    pub encoded_vm_receipt: Vec<u8>,
    pub deterministic: bool,
}
