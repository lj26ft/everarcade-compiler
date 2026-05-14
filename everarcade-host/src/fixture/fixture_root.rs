use execution_core::civilization::CivilizationPackage;

pub type Hash = [u8; 32];

pub fn package_root(package: &CivilizationPackage) -> Hash {
    package.execution_root
}
