use std::{collections::BTreeMap, fs, io::ErrorKind, path::{Path, PathBuf}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArtifactManifest { pub artifact_id: String, pub continuity_root: String, pub sequence: u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArtifactRecord { pub manifest: HistoricalArtifactManifest, pub payload: Vec<u8> }
#[derive(Debug, Clone)]
pub struct HistoricalArtifactStore { root: PathBuf }
#[derive(Debug, Clone)]
pub struct HistoricalArtifactReader { root: PathBuf }
#[derive(Debug, Clone)]
pub struct HistoricalArtifactWriter { root: PathBuf }

impl HistoricalArtifactStore {
    pub fn new(root: impl AsRef<Path>) -> Self { Self { root: root.as_ref().to_path_buf() } }
    pub fn reader(&self) -> HistoricalArtifactReader { HistoricalArtifactReader { root: self.root.clone() } }
    pub fn writer(&self) -> HistoricalArtifactWriter { HistoricalArtifactWriter { root: self.root.clone() } }
}

impl HistoricalArtifactWriter {
    pub fn append(&self, record: &HistoricalArtifactRecord) -> Result<(), String> {
        if record.manifest.artifact_id.is_empty() || record.manifest.continuity_root.is_empty() { return Err("invalid_manifest".into()); }
        fs::create_dir_all(&self.root).map_err(|e| e.to_string())?;
        let path = self.root.join(format!("{:020}_{}.bin", record.manifest.sequence, record.manifest.artifact_id));
        if path.exists() { return Err("append_only_violation".into()); }
        let mut bytes = format!("{}\n{}\n{}\n", record.manifest.artifact_id, record.manifest.continuity_root, record.manifest.sequence).into_bytes();
        bytes.extend_from_slice(&record.payload);
        fs::write(path, bytes).map_err(|e| e.to_string())
    }
}

impl HistoricalArtifactReader {
    pub fn read_all(&self) -> Result<Vec<HistoricalArtifactRecord>, String> {
        match fs::read_dir(&self.root) {
            Ok(entries) => {
                let mut sorted = BTreeMap::new();
                for entry in entries {
                    let entry = entry.map_err(|e| e.to_string())?;
                    if !entry.file_type().map_err(|e| e.to_string())?.is_file() { continue; }
                    let name = entry.file_name().to_string_lossy().to_string();
                    sorted.insert(name, entry.path());
                }
                sorted.into_values().map(|p| read_record(&p)).collect()
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn read_record(path: &Path) -> Result<HistoricalArtifactRecord, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let mut split = bytes.splitn(4, |b| *b == b'\n');
    let artifact_id = String::from_utf8(split.next().ok_or("corrupt_record")?.to_vec()).map_err(|_| "corrupt_record")?;
    let continuity_root = String::from_utf8(split.next().ok_or("corrupt_record")?.to_vec()).map_err(|_| "corrupt_record")?;
    let sequence = String::from_utf8(split.next().ok_or("corrupt_record")?.to_vec()).map_err(|_| "corrupt_record")?.parse::<u64>().map_err(|_| "corrupt_record")?;
    let payload = split.next().unwrap_or(&[]).to_vec();
    Ok(HistoricalArtifactRecord { manifest: HistoricalArtifactManifest { artifact_id, continuity_root, sequence }, payload })
}
