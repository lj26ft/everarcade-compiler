use crate::{reject_replay_mutation, stable_hash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayInspection { pub frame_count: usize, pub continuity_root: String, pub reconstruction_only: bool }

pub fn inspect_replay(frames: &[&str]) -> ReplayInspection {
    ReplayInspection { frame_count: frames.len(), continuity_root: stable_hash(frames), reconstruction_only: true }
}

pub fn restore_checkpoint_visual(checkpoint_root: &str) -> String { stable_hash(&["visual-restore", checkpoint_root]) }

pub fn request_replay_mutation(requested: bool) -> Result<(), &'static str> { reject_replay_mutation(requested) }
