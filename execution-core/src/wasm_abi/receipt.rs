use super::abi::{AbiFuelReport, AbiReceipt};

pub fn fuel_report(limit: u64, used: u64) -> AbiFuelReport {
    AbiFuelReport {
        fuel_limit: limit,
        fuel_used: used,
        exhausted: used >= limit,
    }
}

pub fn validate_receipt_shape(receipt: &AbiReceipt) -> bool {
    receipt.abi_version > 0
}
