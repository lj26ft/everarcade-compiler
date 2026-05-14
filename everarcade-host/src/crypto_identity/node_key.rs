pub type Hash = [u8; 32];

use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeKey { pub secret_root: Hash, pub public_key_root: Hash }

pub fn derive_public_key_root(secret_root: Hash) -> Hash { Sha256::digest(secret_root).into() }
