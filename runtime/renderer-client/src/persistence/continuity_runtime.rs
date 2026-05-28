#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContinuityRuntime;

impl ContinuityRuntime {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
