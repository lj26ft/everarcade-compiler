use crate::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt, receipt_store_error::ReceiptStoreError,
};

pub fn encode_canonical(
    receipt: &DistributedExecutionReceipt,
) -> Result<Vec<u8>, ReceiptStoreError> {
    Ok(bincode::serialize(receipt)?)
}

pub fn decode_canonical(bytes: &[u8]) -> Result<DistributedExecutionReceipt, ReceiptStoreError> {
    Ok(bincode::deserialize(bytes)?)
}
