use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetIdentity {
    pub world_id: String,
    pub asset_id: String,
    pub class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetManifest {
    pub identity: AssetIdentity,
    pub version: u64,
    pub metadata_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetOwnershipRecord {
    pub manifest: AssetManifest,
    pub owner: String,
    pub continuity_counter: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetTransferReceipt {
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetVault {
    pub vault_id: String,
    pub custodian: String,
    pub held_assets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetWitness {
    pub asset_id: String,
    pub ownership_hash: String,
    pub xrpl_anchor: Option<String>,
}
