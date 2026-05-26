#[derive(Debug, Clone, Default)]
pub struct ReplayArchiveHydrationRuntime;

#[derive(Debug, Clone, Default)]
pub struct ReplayArchiveHydrationManifest {
    pub continuity_root: String,
    pub chunk_count: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ReplayArchiveHydrationWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
}
