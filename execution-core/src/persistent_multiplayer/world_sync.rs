use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSyncState {
    pub peer_id: String,
    pub world_root: String,
    pub replay_tip: String,
    pub sync_root: String,
}
impl WorldSyncState {
    pub fn new(peer: &str, world_root: &str, replay_tip: &str) -> Self {
        Self {
            peer_id: peer.into(),
            world_root: world_root.into(),
            replay_tip: replay_tip.into(),
            sync_root: format!("sync:{peer}:{world_root}:{replay_tip}"),
        }
    }
}
