use std::path::PathBuf;

use everarcade_host::{config::HostConfig, receipt_store::read_receipt, run_package_once};
use execution_core::vm::validate_vm_receipt;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), everarcade_host::error::HostError> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("run") => {
            let flag = args.next().ok_or_else(|| everarcade_host::error::HostError::InvalidArgs("missing --package".into()))?;
            if flag != "--package" { return Err(everarcade_host::error::HostError::InvalidArgs("expected --package".into())); }
            let path = PathBuf::from(args.next().ok_or_else(|| everarcade_host::error::HostError::InvalidArgs("missing package path".into()))?);
            let result = run_package_once(HostConfig::new(path, ".everarcade"))?;
            println!("receipt={}", hex::encode(result.receipt.receipt_id));
        }
        Some("verify") => {
            let _flag = args.next();
            let path = PathBuf::from(args.next().ok_or_else(|| everarcade_host::error::HostError::InvalidArgs("missing receipt path".into()))?);
            let receipt = read_receipt(&path)?;
            println!("valid={}", validate_vm_receipt(&receipt));
        }
        Some("anchor-intent") => {
            let _flag = args.next();
            let path = PathBuf::from(args.next().ok_or_else(|| everarcade_host::error::HostError::InvalidArgs("missing receipt path".into()))?);
            let receipt = read_receipt(&path)?;
            println!("receipt_id={}", hex::encode(receipt.receipt_id));
        }
        _ => return Err(everarcade_host::error::HostError::InvalidArgs("usage: run|verify|anchor-intent".into())),
    }
    Ok(())
}
