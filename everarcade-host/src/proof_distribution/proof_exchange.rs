pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofExchangeRecord { pub proof_root: Hash, pub provenance_root: Hash }
