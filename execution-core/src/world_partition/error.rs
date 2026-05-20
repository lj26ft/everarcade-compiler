use thiserror::Error;

#[derive(Debug, Error)]
pub enum PartitionError {
    #[error("region not found: {0}")]
    RegionNotFound(String),
    #[error("invalid migration for entity: {0}")]
    InvalidMigration(String),
    #[error("ownership mismatch for region: {0}")]
    OwnershipMismatch(String),
}
