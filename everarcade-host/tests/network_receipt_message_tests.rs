use everarcade_host::distributed_sync::distributed_receipt_package::DistributedReceiptPackage;
use everarcade_host::network::{
    distributed_receipt_message::DistributedReceiptMessage,
    receipt_message_validation::validate_response_determinism,
    receipt_response::DistributedReceiptResponse,
    receipt_stream::{decode_message, encode_message},
};

#[test]
fn network_receipt_message_is_canonicalizable() {
    let response = DistributedReceiptResponse {
        package_root: [1; 32],
        receipts: vec![DistributedReceiptPackage {
            package_root: [1; 32],
            partition_root: [2; 32],
            receipt_root: [3; 32],
            replay_root: [4; 32],
            checkpoint_root: [5; 32],
            receipt_bytes: vec![1, 2],
        }],
        final_replay_root: [4; 32],
    };
    assert!(validate_response_determinism(&response));
    let bytes = encode_message(&DistributedReceiptMessage::Response(response.clone())).unwrap();
    let decoded = decode_message(&bytes).unwrap();
    assert_eq!(decoded, DistributedReceiptMessage::Response(response));
}
