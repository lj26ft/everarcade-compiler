use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvernodeStateManifest {
    pub package_root: [u8; 32],
    pub vm_receipt_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
    pub ipfs_manifest_root: [u8; 32],
    pub xrpl_anchor_root: [u8; 32],
    pub evernode_instance_root: [u8; 32],
}
