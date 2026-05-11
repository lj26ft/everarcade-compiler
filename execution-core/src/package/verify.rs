use crate::{execute, VmInput};

use super::bundle::{ExecutionPackage, PackageError};

pub fn verify_replay(package: &ExecutionPackage) -> Result<(), PackageError> {
    package.verify_manifest_consistency()?;

    let input = VmInput {
        protocol_epoch_id: package.manifest.protocol_epoch,
        state: package.snapshot.state_entries.clone(),
        plan: package.dag.clone(),
    };
    let output = execute::execute_vm(input);

    if let Some(last) = package.receipts.last() {
        if output.receipt.execution_root != last.execution_root {
            return Err(PackageError::InvalidHash("execution_root"));
        }
    }

    Ok(())
}
