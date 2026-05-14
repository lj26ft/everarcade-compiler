use std::path::Path;

use super::node_manifest::read_node_manifest;

pub fn manifest_needs_repair(state_root: &Path) -> bool {
    match read_node_manifest(state_root) {
        Ok(m) => m.last_receipt_root.is_none() || m.last_checkpoint_root.is_none(),
        Err(_) => true,
    }
}
