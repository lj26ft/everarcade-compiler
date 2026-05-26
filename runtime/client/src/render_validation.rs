use execution_core::render_bridge::{
    projection_roots::derive_projection_root,
    stream::{ProjectionCheckpoint, ProjectionWindow},
};

pub fn verify_render_projection(window: &ProjectionWindow) -> Result<(), String> {
    let derived = derive_projection_root(&window.frames)?.0;
    if derived == window.window_root.0 {
        Ok(())
    } else {
        Err("render projection mismatch".into())
    }
}

pub fn verify_render_frame_chain(frames: &[String]) -> Result<(), String> {
    if frames.windows(2).all(|w| w[0] <= w[1] || !w[1].is_empty()) {
        Ok(())
    } else {
        Err("render frame chain invalid".into())
    }
}

pub fn verify_projection_checkpoint_chain(
    checkpoints: &[ProjectionCheckpoint],
) -> Result<(), String> {
    if checkpoints.windows(2).all(|w| w[0].tick <= w[1].tick) {
        Ok(())
    } else {
        Err("projection checkpoint chain invalid".into())
    }
}

pub fn verify_visual_replay_equivalence(a: &str, b: &str) -> Result<(), String> {
    if a == b {
        Ok(())
    } else {
        Err("visual replay equivalence failed".into())
    }
}
