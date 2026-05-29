#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioProject {
    pub project_id: String,
    pub manifest_hash: String,
    pub replay_root: String,
}

pub fn create_project(project_id: &str, seed: &str) -> StudioProject {
    StudioProject {
        project_id: project_id.to_owned(),
        manifest_hash: crate::stable_hash(&["studio-project", project_id, seed]),
        replay_root: crate::stable_hash(&["replay-root", project_id, seed]),
    }
}

pub fn project_management_diagnostic() -> crate::CreatorDiagnostic {
    crate::diagnostic("studio-project-management", &["project", "manifest", "replay-root"])
}
