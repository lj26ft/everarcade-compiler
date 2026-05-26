use crate::{
    persistence::{artifact::RenderProjectionArtifact, frame_store::ProjectionFrameStore, hash, replay::ProjectionReplayRuntime, session_store::ProjectionSessionStore},
    event_renderer::render_events, hud::render_hud, inventory_renderer::render_inventory,
    stream_transport::load_projection_window, world_renderer::render_world,
};
use execution_core::render_bridge::{
    stream::{ProjectionCheckpoint, ProjectionCursor, ProjectionStreamRuntime, ProjectionWindow},
    RenderFrameEnvelope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderProjectionFrame {
    pub tick: u64,
    pub visual: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderProjectionWindow {
    pub source: ProjectionWindow,
    pub visuals: Vec<RenderProjectionFrame>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RenderProjectionCursor {
    pub next_tick: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProjectionRenderSession {
    pub session_id: String,
    pub rendered_frames: Vec<RenderProjectionFrame>,
    pub checkpoints: Vec<ProjectionCheckpoint>,
    pub cursor: RenderProjectionCursor,
}
#[derive(Debug, Clone, Default)]
pub struct RendererRuntime {
    pub session_store: std::sync::Arc<std::sync::Mutex<ProjectionSessionStore>>,
}


impl RendererRuntime {
    pub fn run_local_projection_demo(&self) -> Result<ProjectionRenderSession, String> {
        let window = load_projection_window()?;
        ProjectionStreamRuntime::verify_projection_window(&window)?;
        let ck = ProjectionStreamRuntime::checkpoint_for(&window)?;
        let visuals = window
            .frames
            .iter()
            .map(|f| self.render_frame(f))
            .collect::<Vec<_>>();
        let mut frame_store = ProjectionFrameStore::default();
        let mut artifacts = Vec::new();
        for (idx, frame) in window.frames.iter().enumerate() {
            let artifact = RenderProjectionArtifact {
                artifact_id: format!("{}-{}", "renderer-local", idx),
                session_id: "renderer-local".into(),
                frame_index: idx as u64,
                projection_root: frame.world.state_root.clone(),
                projection_hash: hash::stable_hash(&frame.world.state_root),
                parent_projection_hash: idx.checked_sub(1).map(|i| hash::stable_hash(&format!("renderer-local-{i}"))),
                event_hashes: vec![hash::stable_hash(&frame.event.root)],
                timestamp: frame.world.tick,
                frame_hash: String::new(),
            }.with_deterministic_hash()?;
            frame_store.persist_projection_frame(artifact.clone())?;
            artifacts.push(artifact);
        }
        self.session_store.lock().map_err(|_| "session lock poisoned")?.persist_projection_session("renderer-local", artifacts.clone());
        let _replay = ProjectionReplayRuntime::new(artifacts);
        Ok(ProjectionRenderSession {
            session_id: "renderer-local".into(),
            rendered_frames: visuals,
            checkpoints: vec![ck],
            cursor: RenderProjectionCursor {
                next_tick: window.end_tick + 1,
            },
        })
    }

    #[allow(dead_code)]
    pub fn project_window(
        &self,
        window: &ProjectionWindow,
        cursor: &ProjectionCursor,
    ) -> Result<RenderProjectionWindow, String> {
        if window.start_tick < cursor.next_tick && cursor.next_tick != 0 {
            return Err("projection order violation".into());
        }
        Ok(RenderProjectionWindow {
            source: window.clone(),
            visuals: window.frames.iter().map(|f| self.render_frame(f)).collect(),
        })
    }

    fn render_frame(&self, frame: &RenderFrameEnvelope) -> RenderProjectionFrame {
        let world = render_world(frame);
        let inv = render_inventory(frame);
        let ev = render_events(frame);
        let hud = render_hud(frame, 0, 0, "checkpoint-local", "live", "renderer-local");
        RenderProjectionFrame {
            tick: frame.world.tick,
            visual: format!("{world}\n{inv}\n{ev}\n{hud}"),
        }
    }
}
