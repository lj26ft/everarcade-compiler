#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayReplayContinuity {
    pub continuity_root: String,
    pub latest_tick: u64,
    pub append_only: bool,
}
