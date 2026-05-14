use execution_core::civilization::{
    civilization_validation::validate_civilization_package, CivilizationPackage,
};

pub fn validate_fixture_package(package: &CivilizationPackage) -> bool {
    validate_civilization_package(package)
}
