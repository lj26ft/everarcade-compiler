use sha2::{Digest, Sha256};

pub fn compute_vm_instance_root(package_root: [u8;32], receipt_root: [u8;32]) -> [u8;32] {
    Sha256::digest([package_root.as_slice(), receipt_root.as_slice()].concat()).into()
}
