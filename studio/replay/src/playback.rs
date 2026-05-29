pub fn playback_hash(frames: &[&str]) -> String { let mut parts = vec!["replay-playback", "reconstruction-only"]; parts.extend_from_slice(frames); crate::stable_hash(&parts) }
pub fn request_replay_mutation(requested: bool) -> Result<(), &'static str> { crate::reject_replay_mutation(requested) }
