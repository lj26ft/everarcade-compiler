#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconciliationPlan {
    pub selected_peer_id: String,
    pub selected_continuity_height: u64,
}
