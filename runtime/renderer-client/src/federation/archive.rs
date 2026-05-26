#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignReplayArchive {
    pub archive_id: String,
    pub root: SovereignReplayArchiveRoot,
    pub manifest: SovereignReplayArchiveManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignReplayArchiveManifest {
    pub continuity_root: String,
    pub shard_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignReplayArchiveRoot {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayShard {
    pub shard_id: String,
    pub start_sequence: u64,
    pub end_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayShardManifest {
    pub shard_id: String,
    pub continuity: ProjectionReplayShardContinuity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayShardContinuity {
    pub previous_root: String,
    pub current_root: String,
}
