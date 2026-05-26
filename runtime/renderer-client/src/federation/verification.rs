#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionAnchorRecord {
    pub anchor_id: String,
    pub window: ProjectionAnchorWindow,
    pub continuity: ProjectionAnchorContinuityRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionAnchorWindow {
    pub start_frame: u64,
    pub end_frame: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionAnchorContinuityRoot {
    pub root: String,
}

pub fn replay_equivalent(source: &[String], observer: &[String]) -> bool { source == observer }
