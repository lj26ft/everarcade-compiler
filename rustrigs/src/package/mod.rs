use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigPackageManifest {
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub record_types: Vec<String>,
    pub protocol_version: String,
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigPackageArtifactSet {
    pub manifest_toml: String,
    pub metadata_json: String,
    pub package_bin: String,
    pub hash_sha256: String,
    pub signature_bin: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageReputation {
    pub downloads: u64,
    pub installs: u64,
    pub validation_passes: u64,
    pub dependency_count: usize,
    pub compatibility_score: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommunityTemplateKind {
    RustrigPackage,
    RustrigBundle,
    GameplayTemplate,
    GameTemplate,
    WorldTemplate,
}

pub fn stable_package_hash(manifest: &RustrigPackageManifest, package_bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for field in [
        manifest.package_id.as_str(),
        manifest.name.as_str(),
        manifest.version.as_str(),
        manifest.author.as_str(),
        manifest.description.as_str(),
        manifest.protocol_version.as_str(),
    ] {
        for byte in field.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for dep in &manifest.dependencies {
        for byte in dep.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    for record in &manifest.record_types {
        for byte in record.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    for byte in package_bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn reproducible(manifest: &RustrigPackageManifest, first: &[u8], second: &[u8]) -> bool {
    stable_package_hash(manifest, first) == stable_package_hash(manifest, second)
}

pub fn required_artifacts_present(artifacts: &RustrigPackageArtifactSet) -> bool {
    !artifacts.manifest_toml.is_empty()
        && !artifacts.metadata_json.is_empty()
        && !artifacts.package_bin.is_empty()
        && !artifacts.hash_sha256.is_empty()
        && !artifacts.signature_bin.is_empty()
}

pub fn arena_vanguard_required_manifests() -> Vec<RustrigPackageManifest> {
    [
        "Combat",
        "Inventory",
        "Quest",
        "Dialogue",
        "Economy",
        "World",
    ]
    .into_iter()
    .map(|name| RustrigPackageManifest {
        package_id: format!("arena-vanguard-{}", name.to_ascii_lowercase()),
        name: name.to_owned(),
        version: "0.1.0".to_owned(),
        author: "everarcade-core".to_owned(),
        description: format!("Arena Vanguard {name} deterministic marketplace module"),
        dependencies: Vec::new(),
        record_types: vec![format!("{name}Record")],
        protocol_version: "everarcade-protocol-1".to_owned(),
        hash: format!("arena-vanguard-{}-hash", name.to_ascii_lowercase()),
    })
    .collect()
}
