use super::{anchor_intent::EvernodeAnchorIntent, host_manifest::HostManifest};

pub fn validate_manifest(m: &HostManifest) -> bool { !m.vm_instance_root_hex.is_empty() }
pub fn validate_anchor_intent(i: &EvernodeAnchorIntent) -> bool { !i.manifest_hash_hex.is_empty() }
