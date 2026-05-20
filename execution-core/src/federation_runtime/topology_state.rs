use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerStatus {
    Active,
    Stale,
    Diverged,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeerContinuityState {
    pub peer_id: [u8; 32],
    pub continuity_epoch: u64,
    pub topology_epoch: u64,
    pub lease_valid: bool,
    pub last_checkpoint: [u8; 32],
    pub status: PeerStatus,
}

impl PeerContinuityState {
    pub fn new(peer_id: [u8; 32]) -> Self {
        Self {
            peer_id,
            continuity_epoch: 1,
            topology_epoch: 1,
            lease_valid: true,
            last_checkpoint: [0u8; 32],
            status: PeerStatus::Stale,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyStateEngine {
    pub continuity_epoch: u64,
    pub peers: Vec<PeerContinuityState>,
}

impl TopologyStateEngine {
    pub fn upsert_peer(&mut self, peer: PeerContinuityState) {
        if let Some(slot) = self.peers.iter_mut().find(|p| p.peer_id == peer.peer_id) {
            *slot = peer;
            return;
        }
        self.peers.push(peer);
        self.peers.sort_by_key(|p| p.peer_id);
    }

    pub fn peer_state(&self, peer_id: [u8; 32]) -> Option<&PeerContinuityState> {
        self.peers.iter().find(|p| p.peer_id == peer_id)
    }
}
