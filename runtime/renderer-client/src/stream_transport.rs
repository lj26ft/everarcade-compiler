use execution_core::{game_runtime::replay_runtime::{ReplayRecord, ReplayTickRecord}, render_bridge::stream::{ProjectionStreamRuntime, ProjectionWindow}};

pub fn load_projection_window() -> Result<ProjectionWindow, String> {
    let replay = ReplayRecord { ticks: (1..=4).map(|t| ReplayTickRecord { tick: t, inputs: vec![], state_root: format!("world-{t}"), event_root: format!("event-{t}"), validation_root: format!("validation-{t}") }).collect() };
    ProjectionStreamRuntime::materialize_projection_window(&replay, 1, 4)
}
