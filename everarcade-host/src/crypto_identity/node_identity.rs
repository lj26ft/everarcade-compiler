pub type Hash = [u8; 32];

use super::peer_fingerprint::derive_peer_fingerprint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CryptographicNodeIdentity {
    pub node_id: Hash,
    pub public_key_root: Hash,
    pub fingerprint_root: Hash,
    pub federation_scope_root: Option<Hash>,
}

pub fn derive_identity(node_id: Hash, public_key_root: Hash, federation_scope_root: Option<Hash>) -> CryptographicNodeIdentity {
    let fingerprint_root = derive_peer_fingerprint(node_id, public_key_root);
    CryptographicNodeIdentity { node_id, public_key_root, fingerprint_root, federation_scope_root }
}
