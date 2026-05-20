use serde::{Deserialize, Serialize};

use super::settlement::SettlementRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SettlementContinuity {
    pub records: Vec<SettlementRecord>,
    pub continuity_root: [u8; 32],
}

pub fn sync_settlement_continuity(
    local: &mut SettlementContinuity,
    incoming: SettlementContinuity,
) {
    local.records.extend(incoming.records);
    local.records.sort_by(|a, b| {
        a.transaction_reference
            .settlement_epoch
            .cmp(&b.transaction_reference.settlement_epoch)
            .then(a.checkpoint_lineage.cmp(&b.checkpoint_lineage))
    });
    local.records.dedup_by(|a, b| {
        a.transaction_reference.transaction_hash == b.transaction_reference.transaction_hash
    });
}

pub fn verify_federated_settlement(continuity: &SettlementContinuity) -> bool {
    continuity.records.windows(2).all(|w| {
        w[0].transaction_reference.settlement_epoch <= w[1].transaction_reference.settlement_epoch
    })
}
