use crate::sync::{ProofExchange, ReceiptRange, SyncRequest, SyncStatus, validate_proof_exchange, validate_receipt_range};

pub fn validate_request_continuity(local: &SyncStatus, request: &SyncRequest) -> bool {
    request.from_index <= local.next_index && request.to_index.is_none_or(|to| to >= request.from_index)
}

pub fn validate_response_proofs(exchange: &ProofExchange) -> bool { validate_proof_exchange(exchange) }

pub fn validate_response_range(range: &ReceiptRange) -> bool { validate_receipt_range(range) }
