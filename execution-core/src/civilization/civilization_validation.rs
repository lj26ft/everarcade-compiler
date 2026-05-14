use super::{
    civilization_package::CivilizationPackage, civilization_root::compute_civilization_root,
};

pub fn validate_civilization_package(pkg: &CivilizationPackage) -> bool {
    let civilization_root = compute_civilization_root(&pkg.genesis);
    civilization_root == pkg.execution_root
}
