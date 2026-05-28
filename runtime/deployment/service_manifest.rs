#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDeploymentManifest {
    pub package: String,
    pub startup_command: String,
    pub recovery_command: String,
    pub health_check_command: String,
    pub append_only_storage: bool,
}

impl RuntimeDeploymentManifest {
    pub fn evernode() -> Self {
        Self {
            package: "everarcade-sovereign-runtime".into(),
            startup_command: "runtime-node-start".into(),
            recovery_command: "runtime-node-start --recover".into(),
            health_check_command: "runtime-health-check".into(),
            append_only_storage: true,
        }
    }
}
