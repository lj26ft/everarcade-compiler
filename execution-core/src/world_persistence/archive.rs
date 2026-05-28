use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayArchive {
    pub entries: Vec<String>,
    pub archive_root: String,
}
impl ReplayArchive {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            archive_root: "archive:genesis".into(),
        }
    }
    pub fn append(&mut self, entry: &str) {
        self.entries.push(entry.into());
        self.archive_root = format!("archive:{}:{}", self.entries.len(), entry)
    }
}
