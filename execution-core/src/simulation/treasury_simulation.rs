use crate::treasury::treasury::Hash;
pub fn simulate_treasury_epoch(root: Hash, epoch: u64) -> Hash {
    let mut o = root;
    o[0..8].copy_from_slice(&epoch.to_le_bytes());
    o
}
