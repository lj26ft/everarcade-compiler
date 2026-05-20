use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettlementError {
    #[error("asset not found: {0}")]
    AssetMissing(String),
    #[error("asset already exists: {0}")]
    AssetExists(String),
    #[error("ownership mismatch for asset {asset_id}: expected {expected}, found {actual}")]
    OwnershipMismatch {
        asset_id: String,
        expected: String,
        actual: String,
    },
    #[error("invalid transaction reference")]
    InvalidTransactionReference,
}
