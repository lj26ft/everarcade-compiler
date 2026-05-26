use super::artifact::RenderProjectionArtifact;

#[derive(Debug, Clone, Default)]
pub struct ProjectionFrameStore {
    frames: Vec<RenderProjectionArtifact>,
}

impl ProjectionFrameStore {
    pub fn persist_projection_frame(
        &mut self,
        frame: RenderProjectionArtifact,
    ) -> Result<(), String> {
        let expected = self.frames.len() as u64;
        if frame.frame_index != expected {
            return Err("append-only ordering violation".into());
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn frames(&self) -> &[RenderProjectionArtifact] {
        &self.frames
    }
}
