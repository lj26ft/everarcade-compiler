use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldContinuityRoot(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityDivergence {
    pub expected: WorldContinuityRoot,
    pub observed: WorldContinuityRoot,
}
