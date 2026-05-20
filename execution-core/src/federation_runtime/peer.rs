use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerAddress(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerCapabilities {
    pub checkpoint_sync: bool,
    pub journal_sync: bool,
    pub replay_verification: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerContinuityState {
    pub continuity_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerIdentity {
    pub node_id: [u8; 32],
    pub continuity_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
    pub protocol_version: u32,
    pub topology_epoch: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FederationPeer {
    pub identity: PeerIdentity,
    pub address: PeerAddress,
    pub capabilities: PeerCapabilities,
    pub continuity_state: PeerContinuityState,
}

impl PeerIdentity {
    pub fn is_ambiguous(&self) -> bool {
        self.protocol_version == 0 || self.topology_epoch == 0
    }
}
