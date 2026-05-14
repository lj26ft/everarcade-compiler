#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaleNodeStatus {
    pub checkpoint_lag: u64,
    pub disconnected: bool,
}
