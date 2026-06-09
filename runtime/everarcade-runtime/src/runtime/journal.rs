use crate::runtime::persistence::PersistenceManager;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JournalEntry {
    pub sequence: u64,
    pub previous_hash: String,
    pub state_root: String,
    pub input_hash: String,
    pub receipt_hash: String,
    pub timestamp_ms: u128,
    pub entry_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gameplay_input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_invocation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_output_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_output: Option<serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct JournalManager {
    pub path: PathBuf,
    persistence: PersistenceManager,
}

impl JournalManager {
    pub fn new(path: PathBuf, persistence: PersistenceManager) -> Self {
        Self { path, persistence }
    }

    pub fn append(
        &self,
        sequence: u64,
        previous_hash: String,
        state_root: String,
        input_hash: String,
        receipt_hash: String,
    ) -> Result<JournalEntry> {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let entry_hash = Self::hash_fields(
            sequence,
            &previous_hash,
            &state_root,
            &input_hash,
            &receipt_hash,
            timestamp_ms,
        );
        let entry = JournalEntry {
            sequence,
            previous_hash,
            state_root,
            input_hash,
            receipt_hash,
            timestamp_ms,
            entry_hash,
            player_id: None,
            action: None,
            tick: None,
            gameplay_input: None,
            guest_id: None,
            guest_hash: None,
            guest_invocation: None,
            guest_output_hash: None,
            guest_output: None,
        };
        self.persistence
            .append_line_fsync(&self.path, &serde_json::to_string(&entry)?)?;
        Ok(entry)
    }

    pub fn entries(&self) -> Result<Vec<JournalEntry>> {
        let data = self.persistence.read_to_string_if_exists(&self.path)?;
        data.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| Ok(serde_json::from_str(l)?))
            .collect()
    }

    pub fn verify(&self) -> Result<Option<JournalEntry>> {
        let entries = self.entries()?;
        let mut previous = GENESIS_HASH.to_string();
        let mut expected_sequence = 1;
        for entry in &entries {
            if entry.sequence != expected_sequence {
                return Err(anyhow!("journal sequence gap at {}", expected_sequence));
            }
            if entry.previous_hash != previous {
                return Err(anyhow!("journal hash chain broken at {}", entry.sequence));
            }
            let expected_hash = Self::hash_fields(
                entry.sequence,
                &entry.previous_hash,
                &entry.state_root,
                &entry.input_hash,
                &entry.receipt_hash,
                entry.timestamp_ms,
            );
            if entry.entry_hash != expected_hash {
                return Err(anyhow!("journal entry hash mismatch at {}", entry.sequence));
            }
            previous = entry.entry_hash.clone();
            expected_sequence += 1;
        }
        Ok(entries.last().cloned())
    }

    pub fn latest_hash(&self) -> Result<String> {
        Ok(self
            .verify()?
            .map(|e| e.entry_hash)
            .unwrap_or_else(|| GENESIS_HASH.to_string()))
    }

    pub fn append_gameplay(
        &self,
        sequence: u64,
        previous_hash: String,
        state_root: String,
        input_hash: String,
        receipt_hash: String,
        player_id: String,
        action: String,
        tick: u64,
        gameplay_input: serde_json::Value,
    ) -> Result<JournalEntry> {
        let timestamp_ms = sequence as u128;
        let entry_hash = Self::hash_fields(
            sequence,
            &previous_hash,
            &state_root,
            &input_hash,
            &receipt_hash,
            timestamp_ms,
        );
        let entry = JournalEntry {
            sequence,
            previous_hash,
            state_root,
            input_hash,
            receipt_hash,
            timestamp_ms,
            entry_hash,
            player_id: Some(player_id),
            action: Some(action),
            tick: Some(tick),
            gameplay_input: Some(gameplay_input),
            guest_id: None,
            guest_hash: None,
            guest_invocation: None,
            guest_output_hash: None,
            guest_output: None,
        };
        self.persistence
            .append_line_fsync(&self.path, &serde_json::to_string(&entry)?)?;
        Ok(entry)
    }


    pub fn append_guest(
        &self,
        sequence: u64,
        previous_hash: String,
        state_root: String,
        input_hash: String,
        receipt_hash: String,
        guest_id: String,
        guest_hash: String,
        guest_output_hash: String,
        guest_invocation: serde_json::Value,
        guest_output: serde_json::Value,
    ) -> Result<JournalEntry> {
        let timestamp_ms = sequence as u128;
        let entry_hash = Self::hash_fields(
            sequence,
            &previous_hash,
            &state_root,
            &input_hash,
            &receipt_hash,
            timestamp_ms,
        );
        let action = guest_output
            .get("action")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let player_id = guest_output
            .get("player_id")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let entry = JournalEntry {
            sequence,
            previous_hash,
            state_root,
            input_hash,
            receipt_hash,
            timestamp_ms,
            entry_hash,
            player_id,
            action,
            tick: Some(sequence),
            gameplay_input: None,
            guest_id: Some(guest_id),
            guest_hash: Some(guest_hash),
            guest_invocation: Some(guest_invocation),
            guest_output_hash: Some(guest_output_hash),
            guest_output: Some(guest_output),
        };
        self.persistence
            .append_line_fsync(&self.path, &serde_json::to_string(&entry)?)?;
        Ok(entry)
    }

    pub fn hash_fields(
        sequence: u64,
        previous_hash: &str,
        state_root: &str,
        input_hash: &str,
        receipt_hash: &str,
        timestamp_ms: u128,
    ) -> String {
        let mut h = Sha256::new();
        h.update(sequence.to_le_bytes());
        h.update(previous_hash.as_bytes());
        h.update(state_root.as_bytes());
        h.update(input_hash.as_bytes());
        h.update(receipt_hash.as_bytes());
        h.update(timestamp_ms.to_le_bytes());
        hex::encode(h.finalize())
    }
}
