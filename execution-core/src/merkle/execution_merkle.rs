use crate::{execution::ExecutionState, receipt_runtime::execution_receipt::ExecutionReceipt};

use super::{leaf_hash::inner_hash, receipt_merkle::receipt_root, state_merkle::state_root, Hash};

pub fn execution_root(state: &ExecutionState, receipts: &[ExecutionReceipt]) -> Hash {
    inner_hash(state_root(state), receipt_root(receipts))
}
