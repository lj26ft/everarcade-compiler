use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityManifest {
    pub capability_id: String,
    pub version: String,
    pub author_id: String,
    pub capability_hash: String,
    pub description: String,
    pub dependencies: Vec<CapabilityDependency>,
    pub license: String,
    pub reward_model: RewardModel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDependency {
    pub capability_id: String,
    pub version_constraint: VersionConstraint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionConstraint {
    Exact(String),
    Compatible(String),
    AtLeast(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RewardModel {
    Free,
    OneTimeLicense,
    Subscription,
    Royalty,
    TreasuryGrant,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityPackage {
    pub artifact_name: String,
    pub manifest: CapabilityManifest,
    pub replay_log_hash: String,
    pub restore_proof_hash: String,
    pub migration_plan_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRegistryEntry {
    pub name: String,
    pub author: String,
    pub version: String,
    pub worlds_using: u64,
    pub rating: u8,
    pub reputation: u64,
    pub reward_model: RewardModel,
    pub verification: VerificationSignals,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationSignals {
    pub replay_verified: bool,
    pub restore_verified: bool,
    pub migration_verified: bool,
    pub security_status: SecurityStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityStatus {
    Audited,
    Warnings(Vec<String>),
    Blocked(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityInstallationRecord {
    pub world_id: String,
    pub capability_id: String,
    pub version: String,
    pub install_root: String,
    pub installed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityReputation {
    pub install_count: u64,
    pub world_adoption: u64,
    pub merge_activity: u64,
    pub contributor_reputation: u64,
    pub governance_participation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityReview {
    pub capability_id: String,
    pub rating: u8,
    pub comment: String,
    pub usage_report: String,
    pub known_issues: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributionRecord {
    pub capability_id: String,
    pub author_id: String,
    pub worlds_using: Vec<String>,
    pub reward_allocations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceAction {
    Install,
    Upgrade,
    Replace,
    Remove,
    Fork,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalModel {
    OperatorApproval,
    CouncilApproval,
    CommunityVote,
    HybridGovernance,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityFork {
    pub original_capability_id: String,
    pub fork_capability_id: String,
    pub maintainers: Vec<String>,
    pub contributors: Vec<String>,
    pub lineage_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCapabilityProfile {
    pub world_id: String,
    pub installed_capabilities: Vec<CapabilityInstallationRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityRegistry {
    entries: BTreeMap<String, CapabilityRegistryEntry>,
    packages: BTreeMap<String, CapabilityPackage>,
}

impl CapabilityRegistry {
    pub fn new(packages: Vec<CapabilityPackage>, entries: Vec<CapabilityRegistryEntry>) -> Self {
        Self {
            packages: packages
                .into_iter()
                .map(|p| (p.manifest.capability_id.clone(), p))
                .collect(),
            entries: entries.into_iter().map(|e| (e.name.clone(), e)).collect(),
        }
    }

    pub fn search(&self, query: &str) -> Vec<&CapabilityRegistryEntry> {
        let query = query.to_ascii_lowercase();
        self.entries
            .values()
            .filter(|entry| entry.name.to_ascii_lowercase().contains(&query))
            .collect()
    }

    pub fn lookup(&self, capability_id: &str) -> Option<&CapabilityPackage> {
        self.packages.get(capability_id)
    }

    pub fn install(
        &self,
        world_id: &str,
        capability_id: &str,
        installed_at: &str,
    ) -> Result<CapabilityInstallationRecord, String> {
        let package = self
            .lookup(capability_id)
            .ok_or_else(|| format!("capability {capability_id} not found"))?;
        let install_root = stable_hash(&[
            "capability-install",
            world_id,
            capability_id,
            &package.manifest.version,
            installed_at,
        ]);
        Ok(CapabilityInstallationRecord {
            world_id: world_id.to_owned(),
            capability_id: capability_id.to_owned(),
            version: package.manifest.version.clone(),
            install_root,
            installed_at: installed_at.to_owned(),
        })
    }

    pub fn resolve_dependencies(&self, roots: &[String]) -> Result<Vec<String>, String> {
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
        capability_id: &str,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        resolved: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(capability_id) {
            return Ok(());
        }
        if !visiting.insert(capability_id.to_owned()) {
            return Err(format!("capability dependency cycle at {capability_id}"));
        }
        let package = self
            .lookup(capability_id)
            .ok_or_else(|| format!("missing capability dependency {capability_id}"))?;
        let mut dependencies = package.manifest.dependencies.clone();
        dependencies.sort_by(|a, b| a.capability_id.cmp(&b.capability_id));
        for dependency in dependencies {
            let dependency_package = self.lookup(&dependency.capability_id).ok_or_else(|| {
                format!("missing capability dependency {}", dependency.capability_id)
            })?;
            if !dependency
                .version_constraint
                .matches(&dependency_package.manifest.version)
            {
                return Err(format!(
                    "capability dependency {} version {} does not satisfy {:?}",
                    dependency.capability_id,
                    dependency_package.manifest.version,
                    dependency.version_constraint
                ));
            }
            self.visit(&dependency.capability_id, visiting, visited, resolved)?;
        }
        visiting.remove(capability_id);
        visited.insert(capability_id.to_owned());
        resolved.push(capability_id.to_owned());
        Ok(())
    }
}

impl VersionConstraint {
    pub fn matches(&self, version: &str) -> bool {
        match self {
            VersionConstraint::Exact(expected) => expected == version,
            VersionConstraint::Compatible(base) => {
                major(base) == major(version) && version >= base.as_str()
            }
            VersionConstraint::AtLeast(minimum) => version >= minimum.as_str(),
        }
    }
}

pub fn verify_capability(package: &CapabilityPackage) -> VerificationSignals {
    let hash_ok = package.manifest.capability_hash.len() == 64
        && package
            .manifest
            .capability_hash
            .chars()
            .all(|c| c.is_ascii_hexdigit());
    VerificationSignals {
        replay_verified: !package.replay_log_hash.is_empty(),
        restore_verified: !package.restore_proof_hash.is_empty(),
        migration_verified: !package.migration_plan_hash.is_empty(),
        security_status: if hash_ok {
            SecurityStatus::Audited
        } else {
            SecurityStatus::Blocked("invalid capability hash".to_owned())
        },
    }
}

pub fn stable_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b"\0");
    }
    hex::encode(hasher.finalize())
}

fn major(version: &str) -> Option<&str> {
    version.split('.').next()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(id: &str, version: &str, deps: Vec<CapabilityDependency>) -> CapabilityPackage {
        CapabilityPackage {
            artifact_name: format!("{id}.evr"),
            manifest: CapabilityManifest {
                capability_id: id.to_owned(),
                version: version.to_owned(),
                author_id: "author-alpha".to_owned(),
                capability_hash: stable_hash(&[id, version]),
                description: format!("{id} capability"),
                dependencies: deps,
                license: "Apache-2.0".to_owned(),
                reward_model: RewardModel::Royalty,
            },
            replay_log_hash: stable_hash(&[id, "replay"]),
            restore_proof_hash: stable_hash(&[id, "restore"]),
            migration_plan_hash: stable_hash(&[id, "migration"]),
        }
    }

    #[test]
    fn installs_capability_with_replayable_record() {
        let housing = package("housing", "1.0.0", vec![]);
        let registry = CapabilityRegistry::new(vec![housing], vec![]);
        let record = registry
            .install("frontier.evr", "housing", "2026-06-21T00:00:00Z")
            .expect("install record");
        assert_eq!(record.capability_id, "housing");
        assert_eq!(record.version, "1.0.0");
        assert_eq!(record.install_root.len(), 64);
    }

    #[test]
    fn resolves_marketplace_dependencies_before_root() {
        let economy = package("economy", "1.2.0", vec![]);
        let governance = package("governance", "1.0.0", vec![]);
        let marketplace = package(
            "marketplace",
            "1.0.0",
            vec![
                CapabilityDependency {
                    capability_id: "economy".to_owned(),
                    version_constraint: VersionConstraint::Compatible("1.0.0".to_owned()),
                },
                CapabilityDependency {
                    capability_id: "governance".to_owned(),
                    version_constraint: VersionConstraint::AtLeast("1.0.0".to_owned()),
                },
            ],
        );
        let registry = CapabilityRegistry::new(vec![marketplace, economy, governance], vec![]);
        assert_eq!(
            registry
                .resolve_dependencies(&["marketplace".to_owned()])
                .unwrap(),
            vec!["economy", "governance", "marketplace"]
        );
    }

    #[test]
    fn detects_dependency_conflicts() {
        let economy = package("economy", "2.0.0", vec![]);
        let marketplace = package(
            "marketplace",
            "1.0.0",
            vec![CapabilityDependency {
                capability_id: "economy".to_owned(),
                version_constraint: VersionConstraint::Compatible("1.0.0".to_owned()),
            }],
        );
        let registry = CapabilityRegistry::new(vec![marketplace, economy], vec![]);
        assert!(registry
            .resolve_dependencies(&["marketplace".to_owned()])
            .is_err());
    }
}
