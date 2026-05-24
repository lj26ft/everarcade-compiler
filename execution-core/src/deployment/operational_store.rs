use super::hash_hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalLedgerCheckpoint {
    pub height: u64,
    pub root: String,
}

pub struct OperationalStore;
pub struct OperationalLedgerWriter;
pub struct OperationalLedgerReader;

impl OperationalStore {
    pub fn append(path: &std::path::Path, entry: &str) {
        let mut entries = OperationalLedgerReader::read(path);
        entries.push(entry.to_string());
        let bytes = bincode::serialize(&entries).expect("serializable");
        std::fs::write(path, bytes).expect("write ledger");
    }
}
impl OperationalLedgerWriter {
    pub fn checkpoint(entries: &[String]) -> OperationalLedgerCheckpoint {
        OperationalLedgerCheckpoint {
            height: entries.len() as u64,
            root: hash_hex(bincode::serialize(entries).expect("serializable")),
        }
    }
}
impl OperationalLedgerReader {
    pub fn read(path: &std::path::Path) -> Vec<String> {
        if !path.exists() {
            return vec![];
        }
        let bytes = std::fs::read(path).expect("read ledger");
        bincode::deserialize(&bytes).expect("deserialize ledger")
    }
}
