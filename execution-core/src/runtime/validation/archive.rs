#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayShardWindow {
    pub shard_id: String,
    pub lower: u64,
    pub upper: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayShardManifest {
    pub continuity_root: String,
    pub windows: Vec<ReplayShardWindow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayShardContinuity {
    pub previous_root: String,
    pub next_root: String,
}

#[derive(Clone, Debug, Default)]
pub struct ReplayShardRuntime;

impl ReplayShardRuntime {
    pub fn validate_continuity(&self, continuity: &ReplayShardContinuity) -> bool {
        !continuity.previous_root.is_empty() && !continuity.next_root.is_empty()
    }
}
