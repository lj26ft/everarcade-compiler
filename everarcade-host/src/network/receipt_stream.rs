use crate::network::distributed_receipt_message::DistributedReceiptMessage;

pub fn encode_message(msg: &DistributedReceiptMessage) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(msg)
}
pub fn decode_message(bytes: &[u8]) -> Result<DistributedReceiptMessage, bincode::Error> {
    bincode::deserialize(bytes)
}
