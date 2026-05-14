use everarcade_host::distributed_receipts::receipt_store::{
    DistributedExecutionReceipt, ReceiptStore,
};

#[test]
fn receipt_persistence_roundtrip() {
    let receipt = DistributedExecutionReceipt::new([1u8; 32], [2u8; 32], [3u8; 32]);
    let mut store = ReceiptStore::default();
    store.put(receipt.clone());
    assert_eq!(store.get(&receipt.receipt_root), Some(&receipt));
}
