use crate::freeze::versions;

use super::checkpoint::SettlementCheckpoint;

pub fn finalize_checkpoint(
    state_root: String,
    snapshot_hash: String,
    receipt_root: String,
    execution_root: String,
    verifier_consensus_hash: String,
) -> SettlementCheckpoint {
    SettlementCheckpoint {
        epoch_id: 1,
        epoch_hash: String::new(),
        transition_hash: None,
        protocol_version: versions::PROTOCOL_VERSION.to_string(),
        state_root,
        snapshot_hash,
        receipt_root,
        execution_root,
        verifier_consensus_hash,
        xrpl_tx_hash: None,
    }
}
