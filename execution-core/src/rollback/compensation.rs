use crate::payload::ExecutionPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationTransition {
    pub prior_receipt: [u8; 32],
    pub compensation_payload: ExecutionPayload,
}
