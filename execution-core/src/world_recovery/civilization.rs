pub fn civilization_recovery_root(civilization_root: &str, replay_tip: &str) -> String {
    format!("recovery:civilization:{civilization_root}:{replay_tip}")
}
