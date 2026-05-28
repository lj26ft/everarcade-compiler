use super::{
    civilization::civilization_recovery_root, entity::entity_recovery_root,
    validation::validate_world_recovery,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignWorldRecovery {
    pub civilization_root: String,
    pub entity_lineage_root: String,
    pub economy_root: String,
    pub inventory_root: String,
    pub replay_tip: String,
    pub recovery_root: String,
}
impl SovereignWorldRecovery {
    pub fn new(c: &str, e: &str, eco: &str, inv: &str, replay: &str) -> Self {
        let recovery_root = format!(
            "world-recovery:{}:{}:{}:{}:{}",
            civilization_recovery_root(c, replay),
            entity_recovery_root(e, replay),
            eco,
            inv,
            replay
        );
        Self {
            civilization_root: c.into(),
            entity_lineage_root: e.into(),
            economy_root: eco.into(),
            inventory_root: inv.into(),
            replay_tip: replay.into(),
            recovery_root,
        }
    }
    pub fn restore(&self) -> Result<Self, &'static str> {
        if validate_world_recovery(self) {
            Ok(self.clone())
        } else {
            Err("corrupted recovery rejected")
        }
    }
}
