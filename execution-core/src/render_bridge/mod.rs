use serde::{Deserialize, Serialize};

pub mod api;
pub mod projection_roots;
pub mod stream;
pub mod validation;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderBoundaryEvent {
    pub tick: u64,
    pub root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderWorldState {
    pub tick: u64,
    pub state_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderReplayAnchor {
    pub replay_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderFrameEnvelope {
    pub event: RenderBoundaryEvent,
    pub world: RenderWorldState,
    pub replay: RenderReplayAnchor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedEntityFrame {
    pub entity_id: u64,
    pub x: i64,
    pub y: i64,
    pub entity_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedInventoryFrame {
    pub owner: String,
    pub inventory_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedEventFrame {
    pub tick: u64,
    pub event_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedWorldFrame {
    pub tick: u64,
    pub world_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedFrameState {
    pub world: ProjectedWorldFrame,
    pub entities: Vec<ProjectedEntityFrame>,
    pub inventory: Vec<ProjectedInventoryFrame>,
    pub events: Vec<ProjectedEventFrame>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderStateProjection {
    pub projection_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationBoundary {
    pub boundary_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationReplayAnchor {
    pub replay_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationProjectionRoot(pub String);
