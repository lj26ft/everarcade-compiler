use super::execution_state::ExecutionState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionOutcome {
    pub node_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionResult {
    pub final_state: ExecutionState,
    pub outcomes: Vec<ExecutionOutcome>,
    pub stable_receipt_order: Vec<String>,
    pub rolled_back: bool,
}
