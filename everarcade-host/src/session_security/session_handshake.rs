pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionHandshake {
    pub local_peer_root: Hash,
    pub remote_peer_root: Hash,
    pub federation_scope_root: Option<Hash>,
    pub capability_roots: Vec<Hash>,
}
