pub type Hash = [u8; 32];
use sha2::{Digest,Sha256};

pub fn derive_signature_root(payload_root: Hash, signer_secret_root: Hash) -> Hash { let mut h=Sha256::new(); h.update(payload_root); h.update(signer_secret_root); h.finalize().into() }

pub fn signature_is_valid(payload_root: Hash, signer_secret_root: Hash, signature_root: Hash) -> bool { derive_signature_root(payload_root, signer_secret_root)==signature_root }
