pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofCacheEntry {
    pub proof_root: Hash,
    pub cached_at_height: u64,
}
