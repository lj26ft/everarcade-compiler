use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineView { pub frames: usize, pub continuity_root: String, pub reconstruction_only: bool }

pub fn inspect_timeline(frames: &[&str]) -> TimelineView { TimelineView { frames: frames.len(), continuity_root: stable_hash(frames), reconstruction_only: true } }
