use std::{fs, path::Path};

use execution_core::runtime_commit::{
    CheckpointRecord, CommitError, ExecutionReceipt, JournalEntry,
};

pub fn persist_commit_records(
    world_root: &Path,
    receipt: &ExecutionReceipt,
    journal: &JournalEntry,
    checkpoint: &CheckpointRecord,
) -> Result<(), String> {
    fs::create_dir_all(world_root.join("receipts")).map_err(|e| e.to_string())?;
    fs::create_dir_all(world_root.join("journal")).map_err(|e| e.to_string())?;
    fs::create_dir_all(world_root.join("checkpoints")).map_err(|e| e.to_string())?;
    atomic_write_json(
        &world_root
            .join("receipts")
            .join(format!("{}.json", hex::encode(receipt.receipt_hash))),
        receipt,
    )?;
    atomic_write_json(
        &world_root
            .join("journal")
            .join(format!("{:020}.json", journal.sequence_number)),
        journal,
    )?;
    atomic_write_json(
        &world_root
            .join("checkpoints")
            .join(format!("{:020}.json", checkpoint.journal_sequence)),
        checkpoint,
    )?;
    Ok(())
}

fn atomic_write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    fs::write(
        &tmp,
        serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn verify_world(world_root: &Path) -> Result<(), String> {
    verify_journal(world_root)?;
    for e in fs::read_dir(world_root.join("journal")).map_err(|e| e.to_string())? {
        let entry = e.map_err(|e| e.to_string())?;
        let j: JournalEntry =
            serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let rpath = world_root
            .join("receipts")
            .join(format!("{}.json", hex::encode(j.receipt_hash)));
        if !rpath.exists() {
            return Err("missing receipt".into());
        }
    }
    Ok(())
}

pub fn verify_journal(world_root: &Path) -> Result<(), String> {
    let mut files: Vec<_> = fs::read_dir(world_root.join("journal"))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    let mut prev = [0u8; 32];
    for (i, p) in files.iter().enumerate() {
        let j: JournalEntry = serde_json::from_slice(&fs::read(p).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if j.sequence_number != i as u64 {
            return Err(CommitError::JournalSequenceMismatch.to_string());
        }
        if j.previous_entry_hash != prev {
            return Err(CommitError::JournalPreviousHashMismatch.to_string());
        }
        prev = j.entry_hash;
    }
    Ok(())
}

pub fn replay_world(world_root: &Path) -> Result<[u8; 32], String> {
    let mut files: Vec<_> = fs::read_dir(world_root.join("checkpoints"))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    let last = files.last().ok_or_else(|| "no checkpoints".to_string())?;
    let c: CheckpointRecord = serde_json::from_slice(&fs::read(last).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(c.state_root)
}
