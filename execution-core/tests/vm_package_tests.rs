use execution_core::package::{
    package_validation::validate_vm_package_manifest, vm_package::VmPackageManifest,
};
#[test]
fn vm_manifest_validates() {
    let m = VmPackageManifest {
        package_id: [1; 32],
        protocol_version: 1,
        civilization_root: [2; 32],
        replay_root: [3; 32],
        checkpoint_root: [4; 32],
    };
    assert!(validate_vm_package_manifest(&m));
}
