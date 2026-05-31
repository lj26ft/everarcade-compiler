use rustrigs::dependency::DependencyGraph;
use rustrigs::package::{reproducible, RustrigPackageManifest};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplacePackage {
    pub manifest: RustrigPackageManifest,
    pub package_bytes: Vec<u8>,
    pub signature: String,
    pub abi_version: String,
    pub deterministic: bool,
    pub replay_safe: bool,
    pub records_valid: bool,
    pub requests_network: bool,
    pub requests_filesystem: bool,
    pub requests_authority_write: bool,
    pub requests_deployment: bool,
    pub requests_xrpl_submission: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub abi_compatible: bool,
    pub deterministic: bool,
    pub records_valid: bool,
    pub replay_safe: bool,
    pub dependencies_valid: bool,
    pub signature_valid: bool,
    pub authority_safe: bool,
}

impl ValidationReport {
    pub fn accepted(&self) -> bool {
        self.abi_compatible
            && self.deterministic
            && self.records_valid
            && self.replay_safe
            && self.dependencies_valid
            && self.signature_valid
            && self.authority_safe
    }
}

pub const MARKETPLACE_SECURITY_RESTRICTIONS: [&str; 5] = [
    "No network access",
    "No filesystem access",
    "No authority writes",
    "No deployment execution",
    "No XRPL submission",
];

pub fn validate_package(package: &MarketplacePackage, graph: &DependencyGraph) -> ValidationReport {
    let roots = vec![package.manifest.package_id.clone()];
    let dependencies_valid = graph.validate_versions() && graph.resolve(&roots).is_ok();
    ValidationReport {
        abi_compatible: package.abi_version == package.manifest.protocol_version,
        deterministic: package.deterministic
            && reproducible(
                &package.manifest,
                &package.package_bytes,
                &package.package_bytes,
            ),
        records_valid: package.records_valid && !package.manifest.record_types.is_empty(),
        replay_safe: package.replay_safe,
        dependencies_valid,
        signature_valid: package.signature == format!("signed:{}", package.manifest.hash),
        authority_safe: !(package.requests_network
            || package.requests_filesystem
            || package.requests_authority_write
            || package.requests_deployment
            || package.requests_xrpl_submission),
    }
}

pub fn reject_invalid_package(
    package: &MarketplacePackage,
    graph: &DependencyGraph,
) -> Result<ValidationReport, ValidationReport> {
    let report = validate_package(package, graph);
    if report.accepted() {
        Ok(report)
    } else {
        Err(report)
    }
}

pub fn install_package(
    package: MarketplacePackage,
    graph: &DependencyGraph,
) -> Result<MarketplacePackage, ValidationReport> {
    reject_invalid_package(&package, graph).map(|_| package)
}

pub fn compose_packages(packages: &[MarketplacePackage]) -> Result<Vec<String>, String> {
    if packages.iter().any(|package| {
        package.requests_authority_write
            || package.requests_network
            || package.requests_filesystem
            || package.requests_deployment
            || package.requests_xrpl_submission
    }) {
        return Err("marketplace composition cannot request runtime authority".to_owned());
    }
    let mut records: Vec<String> = packages
        .iter()
        .flat_map(|package| package.manifest.record_types.clone())
        .collect();
    records.sort();
    Ok(records)
}

pub fn replay_safe_marketplace(packages: &[MarketplacePackage]) -> bool {
    packages
        .iter()
        .all(|package| package.deterministic && package.replay_safe)
        && compose_packages(packages).is_ok()
}
