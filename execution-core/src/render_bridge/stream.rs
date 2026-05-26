use super::{
    projection_roots::*, ProjectedFrameState, RenderBoundaryEvent, RenderFrameEnvelope,
    RenderReplayAnchor, RenderWorldState,
};
use crate::game_runtime::replay_runtime::{ReplayRecord, ReplayTickRecord};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionCursor {
    pub next_tick: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionSequence {
    pub sequence: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionWindow {
    pub start_tick: u64,
    pub end_tick: u64,
    pub frames: Vec<RenderFrameEnvelope>,
    pub window_root: ProjectionWindowRoot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionCheckpoint {
    pub tick: u64,
    pub projection_root: ProjectionRoot,
    pub checkpoint_root: ProjectionCheckpointRoot,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionStreamSession {
    pub cursor: ProjectionCursor,
    pub windows: Vec<ProjectionWindow>,
    pub checkpoints: Vec<ProjectionCheckpoint>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionStreamRuntime;

impl ProjectionStreamRuntime {
    pub fn materialize_projection_window(
        replay: &ReplayRecord,
        start_tick: u64,
        end_tick: u64,
    ) -> Result<ProjectionWindow, String> {
        let frames = replay
            .ticks
            .iter()
            .filter(|t| t.tick >= start_tick && t.tick <= end_tick)
            .map(frame_from_tick)
            .collect::<Vec<_>>();
        let window_root = ProjectionWindowRoot(derive_projection_root(&frames)?.0);
        Ok(ProjectionWindow {
            start_tick,
            end_tick,
            frames,
            window_root,
        })
    }
    pub fn stream_projection_frames(
        window: &ProjectionWindow,
    ) -> impl Iterator<Item = &RenderFrameEnvelope> {
        window.frames.iter()
    }
    pub fn verify_projection_window(window: &ProjectionWindow) -> Result<(), String> {
        let root = ProjectionWindowRoot(derive_projection_root(&window.frames)?.0);
        if root == window.window_root {
            Ok(())
        } else {
            Err("projection window root mismatch".into())
        }
    }
    pub fn resume_projection_stream(
        session: &ProjectionStreamSession,
        from_tick: u64,
    ) -> Vec<RenderFrameEnvelope> {
        session
            .windows
            .iter()
            .flat_map(|w| w.frames.iter())
            .filter(|f| f.event.tick >= from_tick)
            .cloned()
            .collect()
    }
    pub fn checkpoint_for(window: &ProjectionWindow) -> Result<ProjectionCheckpoint, String> {
        let projection_root = derive_projection_root(window)?;
        let checkpoint_root = ProjectionCheckpointRoot(
            derive_projection_root(&(window.end_tick, &projection_root.0))?.0,
        );
        Ok(ProjectionCheckpoint {
            tick: window.end_tick,
            projection_root,
            checkpoint_root,
        })
    }
}

fn frame_from_tick(t: &ReplayTickRecord) -> RenderFrameEnvelope {
    RenderFrameEnvelope {
        event: RenderBoundaryEvent {
            tick: t.tick,
            root: t.event_root.clone(),
        },
        world: RenderWorldState {
            tick: t.tick,
            state_root: t.state_root.clone(),
        },
        replay: RenderReplayAnchor {
            replay_root: t.validation_root.clone(),
        },
    }
}

pub fn materialize_projected_state(frame: &RenderFrameEnvelope) -> ProjectedFrameState {
    ProjectedFrameState {
        world: super::ProjectedWorldFrame {
            tick: frame.world.tick,
            world_root: frame.world.state_root.clone(),
        },
        entities: Vec::new(),
        inventory: Vec::new(),
        events: vec![super::ProjectedEventFrame {
            tick: frame.event.tick,
            event_root: frame.event.root.clone(),
        }],
    }
}
