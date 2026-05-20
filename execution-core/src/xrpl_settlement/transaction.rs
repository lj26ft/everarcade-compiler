use serde::{Deserialize, Serialize};

use super::error::SettlementError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XRPLTransactionReference {
    pub transaction_hash: [u8; 32],
    pub ledger_index: u64,
    pub settlement_epoch: u64,
    pub confirmation_lineage: u64,
}

pub fn register_xrpl_transaction(
    references: &mut Vec<XRPLTransactionReference>,
    reference: XRPLTransactionReference,
) {
    references.push(reference);
    references.sort_by(|a, b| {
        a.settlement_epoch
            .cmp(&b.settlement_epoch)
            .then(a.ledger_index.cmp(&b.ledger_index))
            .then(a.transaction_hash.cmp(&b.transaction_hash))
    });
}

pub fn verify_xrpl_reference(reference: &XRPLTransactionReference) -> Result<(), SettlementError> {
    if reference.transaction_hash == [0u8; 32] || reference.ledger_index == 0 {
        return Err(SettlementError::InvalidTransactionReference);
    }
    Ok(())
}
