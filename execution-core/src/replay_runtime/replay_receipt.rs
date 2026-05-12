use crate::receipt_runtime::execution_receipt::ExecutionReceipt;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayResult { pub converged: bool, pub receipts_applied: usize }
pub fn replay_execution_history(receipts: &[ExecutionReceipt]) -> ReplayResult { ReplayResult { converged: crate::receipt_runtime::receipt_chain::validate_chain(receipts), receipts_applied: receipts.len() } }
