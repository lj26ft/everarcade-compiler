use crate::runtime::configuration::LAYOUT_VERSION;
use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionedEnvelope<T> {
    pub version: u32,
    pub checksum: String,
    pub payload: T,
}

#[derive(Clone, Debug)]
pub struct PersistenceManager {
    pub root: PathBuf,
}

impl PersistenceManager {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn ensure_layout(&self, dirs: &[PathBuf]) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        for d in dirs {
            fs::create_dir_all(d)?;
        }
        Ok(())
    }

    pub fn atomic_write_json<T: Serialize>(
        &self,
        path: impl AsRef<Path>,
        payload: &T,
    ) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let bytes = serde_json::to_vec_pretty(payload)?;
        let tmp = path.with_extension("tmp");
        {
            let mut file = File::create(&tmp)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        fs::rename(&tmp, path)?;
        if let Some(parent) = path.parent() {
            if let Ok(dir) = File::open(parent) {
                let _ = dir.sync_all();
            }
        }
        Ok(())
    }

    pub fn read_json<T: DeserializeOwned>(&self, path: impl AsRef<Path>) -> Result<T> {
        let bytes = fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub fn write_versioned<T: Serialize + Clone>(
        &self,
        path: impl AsRef<Path>,
        payload: &T,
    ) -> Result<()> {
        let payload_bytes = serde_json::to_vec(payload)?;
        let checksum = hex::encode(Sha256::digest(&payload_bytes));
        let envelope = VersionedEnvelope {
            version: LAYOUT_VERSION,
            checksum,
            payload: payload.clone(),
        };
        self.atomic_write_json(path, &envelope)
    }

    pub fn read_versioned<T: DeserializeOwned + Serialize>(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<T> {
        let envelope: VersionedEnvelope<T> = self.read_json(path)?;
        if envelope.version != LAYOUT_VERSION {
            return Err(anyhow!(
                "unsupported runtime layout version {}",
                envelope.version
            ));
        }
        let payload_bytes = serde_json::to_vec(&envelope.payload)?;
        let checksum = hex::encode(Sha256::digest(&payload_bytes));
        if checksum != envelope.checksum {
            return Err(anyhow!("corruption detected: checksum mismatch"));
        }
        Ok(envelope.payload)
    }

    pub fn append_line_fsync(&self, path: impl AsRef<Path>, line: &str) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
        file.sync_all()?;
        Ok(())
    }

    pub fn read_to_string_if_exists(&self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(String::new());
        }
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;
        Ok(s)
    }
}
