use everarcade_host::archive::{
    archive_validation::validate_archive, civilization_archive::package,
};
#[test]
fn archive_continuity() {
    let pkg = package([1; 32], [2; 32], [3; 32], [4; 32]);
    assert!(validate_archive(&pkg));
}
