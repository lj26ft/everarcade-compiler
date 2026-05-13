#[derive(Default, Clone, Debug)]
pub struct PeerRegistry {
    pub peers: Vec<[u8; 32]>,
}
