use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeaseEpoch(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeaseAuthority {
    pub peer_id: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeaseValidity {
    pub continuity_epoch: u64,
    pub topology_epoch: u64,
    pub checkpoint_continuity: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationLease {
    pub epoch: LeaseEpoch,
    pub authority: LeaseAuthority,
    pub peer_identity: [u8; 32],
    pub validity: LeaseValidity,
}
