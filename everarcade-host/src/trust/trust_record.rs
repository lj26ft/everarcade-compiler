pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustRecord { pub subject_root: Hash, pub scope_root: Option<Hash>, pub provenance_root: Hash }
