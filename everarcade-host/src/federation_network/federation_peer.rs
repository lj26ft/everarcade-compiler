#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationPeer { pub peer_id: [u8; 32], pub treaty_root: [u8; 32] }
