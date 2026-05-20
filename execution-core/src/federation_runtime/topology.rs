use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyEpoch(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerStatus {
    Active,
    Stale,
    Diverged,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FederationMembership {
    pub peer_id: [u8; 32],
    pub status: PeerStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyState {
    pub epoch: TopologyEpoch,
    pub members: Vec<FederationMembership>,
}
