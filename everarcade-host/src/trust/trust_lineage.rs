pub type Hash = [u8; 32];
use sha2::{Digest,Sha256}; pub fn extend_trust_lineage(previous_root:Hash, trust_root:Hash)->Hash{let mut h=Sha256::new();h.update(previous_root);h.update(trust_root);h.finalize().into()}
