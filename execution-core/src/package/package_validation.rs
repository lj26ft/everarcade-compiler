use super::vm_package::VmPackageManifest;

pub fn validate_vm_package_manifest(manifest: &VmPackageManifest) -> bool {
    manifest.protocol_version > 0
        && manifest.civilization_root != [0u8; 32]
        && manifest.replay_root != [0u8; 32]
        && manifest.checkpoint_root != [0u8; 32]
}
