use super::{
    civilization::civilization_recovery_root, entity::entity_recovery_root,
    runtime::SovereignWorldRecovery,
};
pub fn validate_world_recovery(r: &SovereignWorldRecovery) -> bool {
    r.recovery_root
        == format!(
            "world-recovery:{}:{}:{}:{}:{}",
            civilization_recovery_root(&r.civilization_root, &r.replay_tip),
            entity_recovery_root(&r.entity_lineage_root, &r.replay_tip),
            r.economy_root,
            r.inventory_root,
            r.replay_tip
        )
}
