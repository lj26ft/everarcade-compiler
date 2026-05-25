#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SandboxBoundary {
    pub strategy: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SandboxWitness {
    pub boundary_id: String,
    pub result: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SandboxValidationSurface {
    pub syscall_filtering_enabled: bool,
    pub wasi_isolation_profile: String,
}
