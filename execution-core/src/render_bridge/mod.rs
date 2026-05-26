use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderBoundaryEvent { pub tick: u64, pub root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderWorldState { pub tick: u64, pub state_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderReplayAnchor { pub replay_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderFrameEnvelope { pub event: RenderBoundaryEvent, pub world: RenderWorldState, pub replay: RenderReplayAnchor }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderStateProjection { pub projection_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationBoundary { pub boundary_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationReplayAnchor { pub replay_root: String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationProjectionRoot(pub String);
