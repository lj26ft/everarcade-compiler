#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimePressureProfile {
    pub stage_count: usize,
    pub memory_ceiling_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationMemoryProfile {
    pub rss_bytes: u64,
}
