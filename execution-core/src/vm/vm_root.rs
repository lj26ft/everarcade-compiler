use sha2::{Digest, Sha256};

use super::vm_input::{Hash, VmExecutionInput};

pub fn compute_vm_input_root(input: &VmExecutionInput) -> Hash {
    let encoded = bincode::serialize(input).unwrap_or_default();
    Sha256::digest(encoded).into()
}
