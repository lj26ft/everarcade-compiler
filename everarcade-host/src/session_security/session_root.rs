pub type Hash = [u8; 32];
use sha2::{Digest,Sha256};
pub fn derive_session_root(local_peer_root: Hash, remote_peer_root: Hash, federation_scope_root: Option<Hash>) -> Hash { let mut h=Sha256::new(); h.update(local_peer_root); h.update(remote_peer_root); if let Some(scope)=federation_scope_root{h.update(scope);} h.finalize().into() }
