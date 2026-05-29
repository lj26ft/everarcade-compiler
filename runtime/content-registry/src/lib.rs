use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub fn stable_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    hex::encode(hasher.finalize())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryDiagnostic {
    pub deterministic: bool,
    pub package_continuity: &'static str,
    pub mutation_policy: &'static str,
}

pub mod runtime {
    include!("runtime.rs");
}
pub mod package {
    include!("package.rs");
}
pub mod manifest {
    include!("manifest.rs");
}
pub mod signature {
    include!("signature.rs");
}
pub mod validation {
    include!("validation.rs");
}
