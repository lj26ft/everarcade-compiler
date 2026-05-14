use sha2::{Digest, Sha256};

use super::workload_partition::Hash;

pub fn derive_partition_root(package_root: Hash, partition_index: u64, input_root: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(package_root);
    hasher.update(partition_index.to_le_bytes());
    hasher.update(input_root);
    hasher.finalize().into()
}
