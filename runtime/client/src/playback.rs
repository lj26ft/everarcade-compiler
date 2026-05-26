use execution_core::render_bridge::{stream::ProjectionWindow, RenderFrameEnvelope};

#[derive(Debug, Clone, Default)]
pub struct ProjectionPlaybackEngine {
    cursor: usize,
    frames: Vec<RenderFrameEnvelope>,
}

impl ProjectionPlaybackEngine {
    pub fn from_window(window: &ProjectionWindow) -> Self {
        Self {
            cursor: 0,
            frames: window.frames.clone(),
        }
    }
    pub fn step(&mut self) -> Option<RenderFrameEnvelope> {
        let frame = self.frames.get(self.cursor).cloned();
        if frame.is_some() {
            self.cursor += 1;
        }
        frame
    }
    pub fn rewind(&mut self) {
        self.cursor = 0;
    }
}
