use super::{continuity::continuity_root, runtime::PersistentMultiplayerRuntime};
pub fn validate_multiplayer(m: &PersistentMultiplayerRuntime) -> bool {
    m.continuity_root == continuity_root(&m.states)
        && m.states
            .iter()
            .all(|s| s.sync_root == format!("sync:{}:{}:{}", s.peer_id, s.world_root, s.replay_tip))
}
pub fn reject_replay_authority_mutation(authority_write: bool) -> Result<(), &'static str> {
    if authority_write {
        Err("replay-derived authority mutation rejected")
    } else {
        Ok(())
    }
}
