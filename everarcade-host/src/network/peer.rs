pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignPeer {
    pub peer_id: Hash,
    pub node_root: Hash,
    pub checkpoint_root: Hash,
    pub federation_root: Option<Hash>,
}
