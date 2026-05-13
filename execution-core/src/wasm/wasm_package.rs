use crate::package::vm_package::VmPackageManifest;

pub fn encode_wasm_package(package: &VmPackageManifest) -> Vec<u8> {
    bincode::serialize(package).unwrap_or_default()
}
