use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RenderBoundaryEvent { pub tick: u64, pub authoritative_state_root: String, pub render_hint: String }
