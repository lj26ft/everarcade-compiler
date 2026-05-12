use everarcade_abi::{ExecutionReceipt, State};

use crate::hash_runtime::{
    canonical_hash::canonical_hash, receipt_hash::receipt_hash, replay_root::replay_root,
    state_hash::state_root,
};

use super::{
    replay_state::apply_state_diff,
    replay_step::TraceStep,
    replay_trace::validate_step_link,
    replay_validator::{validate_parent_link, DivergenceReason},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    pub final_state: State,
    pub final_state_root: String,
    pub trace: Vec<TraceStep>,
    pub divergence: Option<(usize, DivergenceReason)>,
}

pub fn replay_from_genesis(genesis: State, receipts: &[ExecutionReceipt]) -> ReplayResult {
    let mut state = genesis;
    let mut trace = Vec::new();
    let mut divergence = None;

    for (idx, receipt) in receipts.iter().enumerate() {
        if !validate_parent_link(receipts, idx) {
            divergence = Some((idx, DivergenceReason::ParentReceiptMismatch));
            break;
        }

        let prior_root = state_root(&state);
        if prior_root != receipt.previous_state_root {
            divergence = Some((idx, DivergenceReason::PriorRootMismatch));
            break;
        }

        apply_state_diff(&mut state, &receipt.state_changes);
        let next_root = state_root(&state);
        let transition_root = canonical_hash(format!("{prior_root}:{next_root}").as_bytes());
        let receipt_h = receipt_hash(receipt);
        let replay_h = replay_root(&prior_root, &receipt_h, &next_root, idx);

        let step = TraceStep {
            logical_index: idx,
            receipt_hash: receipt_h,
            parent_receipt_hash: receipt.previous_snapshot_hash.clone(),
            prior_state_root: prior_root,
            transition_root,
            next_state_root: next_root.clone(),
            replay_root: replay_h,
        };

        if !validate_step_link(trace.last(), &step) {
            divergence = Some((idx, DivergenceReason::PriorRootMismatch));
            trace.push(step);
            break;
        }

        if next_root != receipt.new_state_root {
            divergence = Some((idx, DivergenceReason::NextRootMismatch));
            trace.push(step);
            break;
        }

        trace.push(step);
    }

    let final_state_root = state_root(&state);
    ReplayResult { final_state: state, final_state_root, trace, divergence }
}
