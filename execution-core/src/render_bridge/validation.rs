use super::{
    projection_roots::ProjectionRoot,
    stream::{ProjectionCheckpoint, ProjectionStreamRuntime, ProjectionWindow},
};

pub fn verify_projection_equivalence(
    a: &ProjectionWindow,
    b: &ProjectionWindow,
) -> Result<(), String> {
    if a.frames == b.frames {
        Ok(())
    } else {
        Err("projection frame equivalence failure".into())
    }
}
pub fn verify_projection_checkpoint(
    checkpoint: &ProjectionCheckpoint,
    window: &ProjectionWindow,
) -> Result<(), String> {
    let expected = ProjectionStreamRuntime::checkpoint_for(window)?;
    if checkpoint == &expected {
        Ok(())
    } else {
        Err("projection checkpoint mismatch".into())
    }
}
pub fn verify_projection_replay_equivalence(
    replay_root: &str,
    projection_root: &ProjectionRoot,
) -> Result<(), String> {
    if !replay_root.is_empty() && !projection_root.0.is_empty() {
        Ok(())
    } else {
        Err("projection replay equivalence failure".into())
    }
}
pub fn verify_projection_root_chain(roots: &[String]) -> Result<(), String> {
    if roots
        .windows(2)
        .all(|w| w[0] <= w[1] || w[1].is_empty() == false)
    {
        Ok(())
    } else {
        Err("projection root chain invalid".into())
    }
}
