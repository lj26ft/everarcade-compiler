use std::collections::BTreeMap;

use super::local_peer::LocalPeer;

#[derive(Default)]
pub struct PeerRegistry {
    peers: BTreeMap<String, LocalPeer>,
}

impl PeerRegistry {
    pub fn register(&mut self, peer: LocalPeer) {
        self.peers.insert(peer.node_id.clone(), peer);
    }

    pub fn all(&self) -> Vec<LocalPeer> {
        self.peers.values().cloned().collect()
    }
}
