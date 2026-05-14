#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttackSurfaceReport { pub malformed_artifacts: u32, pub invalid_signatures: u32, pub divergence_events: u32 }
