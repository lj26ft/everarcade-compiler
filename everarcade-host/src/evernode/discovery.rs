use super::{peer_manifest::EvernodePeerManifest, peer_registry::PeerRegistry};

pub fn discover(registry: &PeerRegistry) -> Vec<EvernodePeerManifest> {
    registry.peers().to_vec()
}
