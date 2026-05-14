use crate::{
    merkle::{leaf_hash::leaf_hash, receipt_merkle::receipt_root},
    sync::{
        sync_result::{SyncFailure, SyncResult},
        sync_validation::{
            validate_request_continuity, validate_response_proofs, validate_response_range,
        },
        ProofExchange, ReceiptRange, SyncRequest, SyncResponse, SyncStatus,
    },
};

pub fn reduce_sync_exchange(
    local: SyncStatus,
    request: SyncRequest,
    response: SyncResponse,
) -> SyncResult {
    if !validate_request_continuity(&local, &request) {
        return SyncResult {
            converged: false,
            final_state_root: local.state_root,
            final_replay_root: local.replay_root,
            final_receipt_root: local.receipt_root,
            failure: Some(SyncFailure::RequestOutOfRange),
        };
    }

    let exchange = ProofExchange {
        state_proofs: response.state_proofs.clone(),
        receipt_proofs: response.receipt_proofs.clone(),
        replay_proof: response.replay_proof.clone(),
        checkpoint: response.checkpoint.clone(),
    };
    if !validate_response_proofs(&exchange) {
        return SyncResult {
            converged: false,
            final_state_root: local.state_root,
            final_replay_root: local.replay_root,
            final_receipt_root: local.receipt_root,
            failure: Some(SyncFailure::InvalidProofExchange),
        };
    }

    let start = response
        .receipts
        .first()
        .map(|r| r.timestamp_index)
        .unwrap_or(request.from_index);
    let end = response
        .receipts
        .last()
        .map(|r| r.timestamp_index)
        .unwrap_or(start);
    let range = ReceiptRange {
        start_index: start,
        end_index: end,
        receipt_root: receipt_root(&response.receipts),
        receipts: response.receipts.clone(),
    };
    if !validate_response_range(&range) {
        return SyncResult {
            converged: false,
            final_state_root: local.state_root,
            final_replay_root: local.replay_root,
            final_receipt_root: local.receipt_root,
            failure: Some(SyncFailure::InvalidReceiptRange),
        };
    }

    let final_receipt_root = receipt_root(&response.receipts);
    let final_replay_root = response
        .receipts
        .last()
        .map(|r| leaf_hash(r.replay_root.as_bytes()))
        .unwrap_or(local.replay_root);
    let final_state_root = response
        .receipts
        .last()
        .map(|r| leaf_hash(r.state_root.as_bytes()))
        .unwrap_or(local.state_root);
    let converged = request.local_replay_root == final_replay_root || response.receipts.is_empty();
    SyncResult {
        converged,
        final_state_root,
        final_replay_root,
        final_receipt_root,
        failure: if converged {
            None
        } else {
            Some(SyncFailure::ReplayRootMismatch)
        },
    }
}
