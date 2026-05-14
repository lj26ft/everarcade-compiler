use super::peer_manifest::EvernodePeerManifest;

#[derive(Default)]
pub struct PeerRegistry {
    peers: Vec<EvernodePeerManifest>,
}

impl PeerRegistry {
    pub fn register(&mut self, peer: EvernodePeerManifest) {
        self.peers.push(peer);
    }
    pub fn peers(&self) -> &[EvernodePeerManifest] {
        &self.peers
    }
}
