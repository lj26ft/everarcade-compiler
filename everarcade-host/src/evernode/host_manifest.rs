use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostManifest {
    pub vm_instance_root_hex: String,
    pub package_root_hex: String,
    pub receipt_root_hex: String,
    pub checkpoint_root_hex: String,
}
