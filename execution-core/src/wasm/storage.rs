use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageMutation {
    pub key: String,
    pub value: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCheckpoint {
    pub height: u64,
    pub root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageContinuityEnvelope {
    pub lineage: Vec<StorageCheckpoint>,
}

#[derive(Default)]
pub struct DeterministicStorageEngine {
    pub data: std::collections::BTreeMap<String, Vec<u8>>,
    pub checkpoints: Vec<StorageCheckpoint>,
}
