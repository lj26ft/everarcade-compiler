#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeDeployment { pub deployment_hash: String, pub compatible: bool }
pub fn deploy_runtime(package_hash: &str, runtime_version: &str) -> RuntimeDeployment { RuntimeDeployment { deployment_hash: crate::stable_hash(&["runtime-deploy", package_hash, runtime_version]), compatible: runtime_version.starts_with("everarcade-") } }
