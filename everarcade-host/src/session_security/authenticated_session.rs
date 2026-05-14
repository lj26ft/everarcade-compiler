pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedSession {
    pub session_root: Hash,
    pub local_peer_root: Hash,
    pub remote_peer_root: Hash,
    pub federation_scope_root: Option<Hash>,
}
