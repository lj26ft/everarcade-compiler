use serde::{Deserialize, Serialize};

use super::{
    asset::AssetLineage, ledger::LedgerCheckpoint, proof::SettlementProof,
    transaction::XRPLTransactionReference,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementRecord {
    pub world_continuity_root: [u8; 32],
    pub checkpoint_lineage: u64,
    pub transaction_reference: XRPLTransactionReference,
    pub settlement_proof: SettlementProof,
    pub asset_lineage: AssetLineage,
}

pub fn create_settlement_record(
    world_continuity_root: [u8; 32],
    checkpoint: LedgerCheckpoint,
    transaction_reference: XRPLTransactionReference,
    settlement_proof: SettlementProof,
    asset_lineage: AssetLineage,
) -> SettlementRecord {
    SettlementRecord {
        world_continuity_root,
        checkpoint_lineage: checkpoint.ledger_index,
        transaction_reference,
        settlement_proof,
        asset_lineage,
    }
}
