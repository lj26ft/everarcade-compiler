use std::{fs, path::Path};

use crate::vm::VmExecutionReceipt;

use super::errors::PersistenceError;

pub fn save_receipt(path: &Path, receipt: &VmExecutionReceipt) -> Result<(), PersistenceError> {
    let encoded = bincode::serialize(receipt).map_err(PersistenceError::Encode)?;
    fs::write(path, encoded)?;
    Ok(())
}

pub fn load_receipt(path: &Path) -> Result<VmExecutionReceipt, PersistenceError> {
    let bytes = fs::read(path)?;
    bincode::deserialize(&bytes).map_err(PersistenceError::Decode)
}
