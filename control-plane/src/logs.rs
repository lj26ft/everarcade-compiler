use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogKind {
    Deployment,
    Runtime,
    Recovery,
    OperatorAction,
    LeaseEvent,
    AnchorEvent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp_ms: u64,
    pub kind: LogKind,
    pub subject: String,
    pub fields: Vec<(String, String)>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LogStore {
    pub entries: Vec<LogEntry>,
}
impl LogStore {
    pub fn append(
        &mut self,
        timestamp_ms: u64,
        kind: LogKind,
        subject: impl Into<String>,
        fields: Vec<(String, String)>,
    ) {
        self.entries.push(LogEntry {
            timestamp_ms,
            kind,
            subject: subject.into(),
            fields,
        });
    }
    pub fn search(&self, term: &str) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| {
                e.subject.contains(term)
                    || e.fields
                        .iter()
                        .any(|(k, v)| k.contains(term) || v.contains(term))
            })
            .collect()
    }
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_else(|_| "[]".into())
    }
}
