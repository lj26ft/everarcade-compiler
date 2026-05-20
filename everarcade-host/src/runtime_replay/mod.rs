pub mod error;

use std::path::Path;

use execution_core::runtime_commit::{CheckpointRecord, ExecutionReceipt, JournalEntry};

use crate::runtime_state::{self, RuntimeStateStore};

pub use error::RuntimeReplayError;

pub fn verify_journal_chain(world_root: &Path) -> Result<Vec<JournalEntry>, RuntimeReplayError> {
    let mut files: Vec<_> = std::fs::read_dir(world_root.join("journal"))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    let mut out = Vec::new();
    let mut prev = [0u8; 32];
    for (i, p) in files.iter().enumerate() {
        let j: JournalEntry = serde_json::from_slice(&std::fs::read(p)?)?;
        if j.sequence_number != i as u64 {
            return Err(RuntimeReplayError::Invalid(
                "duplicate/missing sequence".into(),
            ));
        }
        if j.previous_entry_hash != prev {
            return Err(RuntimeReplayError::Invalid(
                "broken journal hash chain".into(),
            ));
        }
        prev = j.entry_hash;
        out.push(j);
    }
    Ok(out)
}

pub fn verify_receipts(
    world_root: &Path,
    journal: &[JournalEntry],
) -> Result<Vec<ExecutionReceipt>, RuntimeReplayError> {
    let mut receipts = Vec::new();
    for j in journal {
        let path = world_root
            .join("receipts")
            .join(format!("{}.json", hex::encode(j.receipt_hash)));
        if !path.exists() {
            return Err(RuntimeReplayError::Invalid(
                "missing receipt artifacts".into(),
            ));
        }
        let r: ExecutionReceipt = serde_json::from_slice(&std::fs::read(path)?)?;
        if r.receipt_hash != r.immutable_hash() {
            return Err(RuntimeReplayError::Invalid("modified receipt".into()));
        }
        receipts.push(r);
    }
    Ok(receipts)
}

pub fn verify_checkpoints(world_root: &Path) -> Result<Vec<CheckpointRecord>, RuntimeReplayError> {
    let mut files: Vec<_> = std::fs::read_dir(world_root.join("checkpoints"))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    let mut out = Vec::new();
    let mut prev_seq = None;
    for p in files {
        let c: CheckpointRecord = serde_json::from_slice(&std::fs::read(p)?)?;
        if let Some(prev) = prev_seq {
            if c.journal_sequence <= prev {
                return Err(RuntimeReplayError::Invalid(
                    "invalid checkpoint linkage".into(),
                ));
            }
        }
        prev_seq = Some(c.journal_sequence);
        out.push(c);
    }
    Ok(out)
}

pub fn reconstruct_state_root(
    receipts: &[ExecutionReceipt],
) -> Result<[u8; 32], RuntimeReplayError> {
    receipts
        .last()
        .map(|r| r.new_state_root)
        .ok_or_else(|| RuntimeReplayError::Invalid("empty replay".into()))
}

pub fn replay_world(world_root: &Path) -> Result<[u8; 32], RuntimeReplayError> {
    let journal = verify_journal_chain(world_root)?;
    let receipts = verify_receipts(world_root, &journal)?;
    let _checkpoints = verify_checkpoints(world_root)?;
    reconstruct_state_root(&receipts)
}

pub fn verify_world(world_root: &Path) -> Result<(), RuntimeReplayError> {
    let _ = replay_world(world_root)?;
    Ok(())
}

pub fn verify_world_integrity(world_root: &Path) -> Result<(), RuntimeReplayError> {
    verify_world(world_root)
}
pub fn verify_execution_continuity(world_root: &Path) -> Result<(), RuntimeReplayError> {
    verify_world(world_root)
}
pub fn detect_replay_divergence(world_root: &Path) -> Result<bool, RuntimeReplayError> {
    let replay_root = replay_world(world_root)?;
    let state: RuntimeStateStore = runtime_state::load_state(world_root)?;
    let persisted = runtime_state::persist_state(world_root, &state)?;
    Ok(replay_root != persisted)
}
