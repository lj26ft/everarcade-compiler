use crate::checkpoint::checkpoint_snapshot::CheckpointSnapshot;

pub fn encode_snapshot(snapshot: &CheckpointSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&snapshot.checkpoint_root);
    out.extend_from_slice(&snapshot.state_root);
    out.extend_from_slice(&snapshot.receipt_root);
    out.extend_from_slice(&snapshot.replay_root);
    out.extend_from_slice(&snapshot.last_receipt_hash);
    out.extend_from_slice(&snapshot.logical_index.to_be_bytes());
    out.extend_from_slice(&(snapshot.encoded_state.len() as u64).to_be_bytes());
    out.extend_from_slice(&snapshot.encoded_state);
    out
}
