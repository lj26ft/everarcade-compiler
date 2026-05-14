pub type Hash = [u8; 32];
pub fn proof_valid(proof_root: Hash) -> bool { proof_root != [0; 32] }
