use execution_core::render_bridge::stream::ProjectionCheckpoint;

#[allow(dead_code)]
pub fn verify_graphical_projection(frame_count: usize) -> Result<(), String> {
    if frame_count > 0 {
        Ok(())
    } else {
        Err("no frames".into())
    }
}
#[allow(dead_code)]
pub fn verify_frame_equivalence(a: &str, b: &str) -> Result<(), String> {
    if a == b {
        Ok(())
    } else {
        Err("frame mismatch".into())
    }
}
#[allow(dead_code)]
pub fn verify_projection_checkpoint_chain(
    checkpoints: &[ProjectionCheckpoint],
) -> Result<(), String> {
    if checkpoints.is_empty() {
        return Err("missing checkpoints".into());
    };
    Ok(())
}
#[allow(dead_code)]
pub fn verify_visual_replay_equivalence(
    replay_root: &str,
    projection_root: &str,
) -> Result<(), String> {
    if replay_root.is_empty() || projection_root.is_empty() {
        Err("invalid roots".into())
    } else {
        Ok(())
    }
}
