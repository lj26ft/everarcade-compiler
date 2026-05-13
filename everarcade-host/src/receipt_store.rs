use std::{fs, path::Path};

use execution_core::vm::VmExecutionReceipt;

use crate::error::HostError;

pub fn write_receipt(dir: &Path, receipt: &VmExecutionReceipt) -> Result<std::path::PathBuf, HostError> {
    let name = format!("{}.bin", hex::encode(receipt.receipt_id));
    let path = dir.join(name);
    fs::write(&path, bincode::serialize(receipt).map_err(HostError::Encode)?)?;
    Ok(path)
}

pub fn read_receipt(path: &Path) -> Result<VmExecutionReceipt, HostError> {
    let bytes = fs::read(path)?;
    bincode::deserialize(&bytes).map_err(HostError::Decode)
}
