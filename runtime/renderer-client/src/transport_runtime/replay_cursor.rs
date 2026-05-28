#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayCursor {
    pub next_sequence: u64,
    pub continuity_root: String,
}
