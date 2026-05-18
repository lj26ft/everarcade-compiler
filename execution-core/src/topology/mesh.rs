use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256, sync::cursor::SyncCursor,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ObserverMesh {
    pub observers: BTreeMap<FederationNodeId, ObserverNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ObserverNode {
    pub node_id: FederationNodeId,
    pub latest_cursor: SyncCursor,
    pub synchronized: bool,
}

pub fn mesh_hash(mesh: &ObserverMesh) -> Hash256 {
    Sha256::digest(&canonical_encode(mesh).expect("mesh encode")).into()
}
