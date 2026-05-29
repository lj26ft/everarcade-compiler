use crate::{diagnostic, reject_replay_mutation, replay_visualizer, CreatorDiagnostic};

pub fn replay_visualizer_diagnostic() -> CreatorDiagnostic { diagnostic("replay-visualizer", &["timeline", "window", "checkpoint", "divergence"] ) }

pub fn replay_visualizer_equivalence(frames: &[&str]) -> bool { replay_visualizer::timeline::inspect_timeline(frames) == replay_visualizer::timeline::inspect_timeline(frames) }

pub fn request_replay_mutation(requested: bool) -> Result<(), &'static str> { reject_replay_mutation(requested) }
