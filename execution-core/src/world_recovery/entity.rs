pub fn entity_recovery_root(entity_lineage: &str, replay_tip: &str) -> String {
    format!("recovery:entity:{entity_lineage}:{replay_tip}")
}
