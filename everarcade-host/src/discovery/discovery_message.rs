#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryMessage {
    pub peer_id: [u8; 32],
    pub available: bool,
}
