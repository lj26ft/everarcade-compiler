use super::artifact::RenderProjectionArtifact;

#[derive(Debug, Clone)]
pub struct ProjectionReplayWindow {
    pub start_frame: u64,
    pub end_frame: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectionReplayCursor {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct ProjectionReplayRuntime {
    frames: Vec<RenderProjectionArtifact>,
}

impl ProjectionReplayRuntime {
    pub fn new(frames: Vec<RenderProjectionArtifact>) -> Self {
        Self { frames }
    }

    pub fn seek(&self, frame_index: u64) -> Option<ProjectionReplayCursor> {
        self.frames
            .iter()
            .position(|f| f.frame_index == frame_index)
            .map(|index| ProjectionReplayCursor { index })
    }

    pub fn step(&self, cursor: &mut ProjectionReplayCursor) -> Option<RenderProjectionArtifact> {
        let frame = self.frames.get(cursor.index).cloned();
        if frame.is_some() {
            cursor.index += 1;
        }
        frame
    }

    pub fn window(&self, window: &ProjectionReplayWindow) -> Vec<RenderProjectionArtifact> {
        self.frames
            .iter()
            .filter(|f| f.frame_index >= window.start_frame && f.frame_index <= window.end_frame)
            .cloned()
            .collect()
    }
}
