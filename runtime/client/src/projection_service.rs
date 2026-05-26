use execution_core::{
    game_runtime::replay_runtime::ReplayRecord,
    render_bridge::{
        projection_roots::derive_projection_root,
        stream::{ProjectionCheckpoint, ProjectionStreamRuntime, ProjectionWindow},
        validation::verify_projection_checkpoint,
    },
};

#[allow(dead_code)]
#[derive(Default)]
pub struct RuntimeProjectionService;

#[allow(dead_code)]
impl RuntimeProjectionService {
    pub fn project(
        &self,
        replay: &ReplayRecord,
        start_tick: u64,
        end_tick: u64,
    ) -> Result<ProjectionWindow, String> {
        ProjectionStreamRuntime::materialize_projection_window(replay, start_tick, end_tick)
    }
    pub fn frame_root(window: &ProjectionWindow) -> Result<String, String> {
        Ok(derive_projection_root(&window.frames)?.0)
    }
    pub fn checkpoint(window: &ProjectionWindow) -> Result<ProjectionCheckpoint, String> {
        ProjectionStreamRuntime::checkpoint_for(window)
    }
    pub fn verify_continuity(
        window: &ProjectionWindow,
        checkpoint: &ProjectionCheckpoint,
    ) -> Result<(), String> {
        verify_projection_checkpoint(checkpoint, window)
    }
}
