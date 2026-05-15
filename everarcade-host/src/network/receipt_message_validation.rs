use crate::network::receipt_response::DistributedReceiptResponse;

pub fn validate_response_determinism(response: &DistributedReceiptResponse) -> bool {
    response
        .receipts
        .windows(2)
        .all(|w| w[0].receipt_root <= w[1].receipt_root)
}
