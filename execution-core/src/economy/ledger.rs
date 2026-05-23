use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomicLedgerEntry {
    pub sequence: u64,
    pub asset_id: String,
    pub owner_before: String,
    pub owner_after: String,
    pub vault_hash: String,
    pub settlement_hash: String,
    pub ledger_hash: String,
}

impl EconomicLedgerEntry {
    pub fn new(
        sequence: u64,
        asset_id: String,
        owner_before: String,
        owner_after: String,
        vault_hash: String,
        settlement_hash: String,
    ) -> Self {
        let ledger_hash = hash_bytes(
            format!(
                "{sequence}|{asset_id}|{owner_before}|{owner_after}|{vault_hash}|{settlement_hash}"
            )
            .as_bytes(),
        );
        Self {
            sequence,
            asset_id,
            owner_before,
            owner_after,
            vault_hash,
            settlement_hash,
            ledger_hash,
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let expected = hash_bytes(
            format!(
                "{}|{}|{}|{}|{}|{}",
                self.sequence,
                self.asset_id,
                self.owner_before,
                self.owner_after,
                self.vault_hash,
                self.settlement_hash
            )
            .as_bytes(),
        );
        if self.ledger_hash != expected {
            return Err("ledger hash mismatch".into());
        }
        Ok(())
    }
}

pub fn ledger_root(entries: &[EconomicLedgerEntry]) -> Result<String, String> {
    for entry in entries {
        entry.verify()?;
    }
    Ok(hash_bytes(
        &canonical_encode(&entries.to_vec()).map_err(|e| e.to_string())?,
    ))
}
