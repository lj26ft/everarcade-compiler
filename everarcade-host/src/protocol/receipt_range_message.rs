pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReceiptRangeMessage {
    pub roots: Vec<Hash>,
}
