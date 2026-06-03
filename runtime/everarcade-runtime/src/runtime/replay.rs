use crate::runtime::journal::JournalEntry;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayReport {
    pub entries_replayed: u64,
    pub expected_root: String,
    pub actual_root: String,
    pub diverged: bool,
    pub duration_ms: u128,
}

#[derive(Clone, Debug, Default)]
pub struct ReplayManager;

impl ReplayManager {
    pub fn replay_root(initial_state: &[u8], entries: &[JournalEntry]) -> String {
        let mut state = initial_state.to_vec();
        for entry in entries {
            state.extend_from_slice(entry.input_hash.as_bytes());
            state.extend_from_slice(entry.receipt_hash.as_bytes());
        }
        hex::encode(Sha256::digest(&state))
    }

    pub fn verify_equivalence(
        &self,
        initial_state: &[u8],
        entries: &[JournalEntry],
        expected_root: &str,
    ) -> Result<ReplayReport> {
        let start = Instant::now();
        let actual_root = Self::replay_root(initial_state, entries);
        let diverged = actual_root != expected_root;
        let report = ReplayReport {
            entries_replayed: entries.len() as u64,
            expected_root: expected_root.to_string(),
            actual_root,
            diverged,
            duration_ms: start.elapsed().as_millis(),
        };
        if report.diverged {
            return Err(anyhow!(
                "replay divergence detected: expected {}, actual {}",
                report.expected_root,
                report.actual_root
            ));
        }
        Ok(report)
    }

    pub fn report(
        &self,
        initial_state: &[u8],
        entries: &[JournalEntry],
        expected_root: &str,
    ) -> ReplayReport {
        let start = Instant::now();
        let actual_root = Self::replay_root(initial_state, entries);
        ReplayReport {
            entries_replayed: entries.len() as u64,
            expected_root: expected_root.to_string(),
            diverged: actual_root != expected_root,
            actual_root,
            duration_ms: start.elapsed().as_millis(),
        }
    }
}
