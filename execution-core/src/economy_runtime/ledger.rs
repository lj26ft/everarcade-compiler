use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub seq: u64,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub previous_root: String,
    pub entry_root: String,
}
pub fn next_entry(seq: u64, from: &str, to: &str, amount: u64, previous_root: &str) -> LedgerEntry {
    LedgerEntry {
        seq,
        from: from.into(),
        to: to.into(),
        amount,
        previous_root: previous_root.into(),
        entry_root: format!("ledger:{seq}:{from}:{to}:{amount}:{previous_root}"),
    }
}
