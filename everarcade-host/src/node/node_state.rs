#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Initializing,
    Ready,
    Executing,
    Publishing,
    Anchoring,
    Recovering,
    Error,
}
