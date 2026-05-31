use crate::package::RustrigPackageManifest;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub packages: BTreeMap<String, RustrigPackageManifest>,
}

impl DependencyGraph {
    pub fn new(packages: Vec<RustrigPackageManifest>) -> Self {
        Self {
            packages: packages
                .into_iter()
                .map(|package| (package.package_id.clone(), package))
                .collect(),
        }
    }

    pub fn validate_versions(&self) -> bool {
        self.packages.values().all(|package| {
            !package.version.trim().is_empty()
                && package.version.split('.').count() == 3
                && package.protocol_version == "everarcade-protocol-1"
        })
    }

    pub fn resolve(&self, roots: &[String]) -> Result<Vec<String>, String> {
        let mut resolved = Vec::new();
        let mut visiting = BTreeSet::new();
        let mut visited = BTreeSet::new();
        for root in roots {
            self.visit(root, &mut visiting, &mut visited, &mut resolved)?;
        }
        Ok(resolved)
    }

    fn visit(
        &self,
        package_id: &str,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        resolved: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(package_id) {
            return Ok(());
        }
        if !visiting.insert(package_id.to_owned()) {
            return Err(format!("dependency cycle rejected at {package_id}"));
        }
        let package = self
            .packages
            .get(package_id)
            .ok_or_else(|| format!("missing dependency {package_id}"))?;
        let mut deps = package.dependencies.clone();
        deps.sort();
        for dep in deps {
            self.visit(&dep, visiting, visited, resolved)?;
        }
        visiting.remove(package_id);
        visited.insert(package_id.to_owned());
        resolved.push(package_id.to_owned());
        Ok(())
    }

    pub fn reproducible_graph_hash(&self, roots: &[String]) -> Result<String, String> {
        let resolved = self.resolve(roots)?;
        let mut hash: u64 = 0xcbf29ce484222325;
        for id in resolved {
            let package = &self.packages[&id];
            for field in [id.as_str(), package.version.as_str(), package.hash.as_str()] {
                for byte in field.as_bytes() {
                    hash ^= u64::from(*byte);
                    hash = hash.wrapping_mul(0x100000001b3);
                }
            }
        }
        Ok(format!("{hash:016x}"))
    }
}
