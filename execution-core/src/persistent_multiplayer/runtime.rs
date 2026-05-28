use super::{
    continuity::continuity_root, validation::validate_multiplayer, world_sync::WorldSyncState,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistentMultiplayerRuntime {
    pub states: Vec<WorldSyncState>,
    pub continuity_root: String,
}
impl PersistentMultiplayerRuntime {
    pub fn new(states: Vec<WorldSyncState>) -> Self {
        let continuity_root = continuity_root(&states);
        Self {
            states,
            continuity_root,
        }
    }
    pub fn sync(&mut self, world_root: &str, replay_tip: &str) -> Result<(), &'static str> {
        for s in &mut self.states {
            *s = WorldSyncState::new(&s.peer_id, world_root, replay_tip)
        }
        self.continuity_root = continuity_root(&self.states);
        if validate_multiplayer(self) {
            Ok(())
        } else {
            Err("divergent world synchronization rejected")
        }
    }
}
