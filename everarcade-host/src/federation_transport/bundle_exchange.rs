use execution_core::federation_runtime::bundle::{
    CheckpointBundle, ContinuityBundle, JournalBundle, ReceiptBundle, ReplayProofBundle,
};

use super::error::FederationTransportError;

pub fn request_checkpoint_bundle(
    peer: &ContinuityBundle,
) -> Result<CheckpointBundle, FederationTransportError> {
    validate_bundle(peer)?;
    Ok(CheckpointBundle {
        checkpoint_hash: peer.checkpoint_hash,
        checkpoint_root: peer.state_root,
    })
}

pub fn request_journal_bundle(
    peer: &ContinuityBundle,
) -> Result<JournalBundle, FederationTransportError> {
    validate_bundle(peer)?;
    Ok(JournalBundle {
        journal_hash: peer.journal_hash,
    })
}

pub fn request_receipt_bundle(
    peer: &ContinuityBundle,
) -> Result<ReceiptBundle, FederationTransportError> {
    validate_bundle(peer)?;
    Ok(ReceiptBundle {
        receipt_hashes: peer.receipt_hashes.clone(),
        execution_hashes: peer.execution_hashes.clone(),
    })
}

pub fn request_replay_proof_bundle(
    peer: &ContinuityBundle,
) -> Result<ReplayProofBundle, FederationTransportError> {
    validate_bundle(peer)?;
    Ok(ReplayProofBundle {
        state_root: peer.state_root,
        continuity_hash: peer.continuity_hash,
    })
}

fn validate_bundle(bundle: &ContinuityBundle) -> Result<(), FederationTransportError> {
    if bundle.receipt_hashes.len() != bundle.execution_hashes.len() {
        return Err(FederationTransportError::MalformedBundle);
    }
    Ok(())
}
