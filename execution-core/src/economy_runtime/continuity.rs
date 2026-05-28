pub fn economy_continuity_root(tick: u64, ledger_root: &str) -> String {
    format!("economy:continuity:{tick}:{ledger_root}")
}
