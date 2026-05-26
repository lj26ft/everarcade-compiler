#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveImporter;
impl HistoricalArchiveImporter {
    pub fn import(continuity_root: &str, payload: &[u8]) -> Result<String, String> {
        if payload.is_empty() {
            Err("corrupt_archive".into())
        } else {
            Ok(continuity_root.into())
        }
    }
}
