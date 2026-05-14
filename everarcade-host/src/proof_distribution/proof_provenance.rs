pub type Hash = [u8; 32];
pub fn proof_provenance_matches(proof_root: Hash, provenance_root: Hash) -> bool {
    proof_root == provenance_root
}
