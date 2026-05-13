use super::peer::SovereignPeer;

pub fn validate_peer(peer: &SovereignPeer) -> bool {
    peer.peer_id != [0; 32] && peer.node_root != [0; 32] && peer.checkpoint_root != [0; 32]
}
