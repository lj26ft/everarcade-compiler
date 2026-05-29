#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationLaunch {
    pub launch_hash: String,
    pub projection_only_viewport: bool,
    pub deterministic_runtime_authority: bool,
}

pub fn launch_simulation(project_hash: &str, runtime_version: &str) -> SimulationLaunch {
    SimulationLaunch {
        launch_hash: crate::stable_hash(&["studio-launch", project_hash, runtime_version]),
        projection_only_viewport: true,
        deterministic_runtime_authority: true,
    }
}

pub fn request_authority_bypass(requested: bool) -> Result<(), &'static str> { crate::reject_authority_bypass(requested) }
