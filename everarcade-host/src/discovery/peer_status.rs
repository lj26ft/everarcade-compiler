#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerStatus {
    pub available: bool,
    pub checkpoint_root: [u8; 32],
}
