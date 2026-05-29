use crate::studio::{project, runtime, session, workspace};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioApp {
    pub workspace_hash: String,
    pub project_hash: String,
    pub session_hash: String,
    pub launch_hash: String,
}

pub fn launch_studio(project_id: &str, seed: &str) -> StudioApp {
    let project = project::create_project(project_id, seed);
    let workspace = workspace::open_workspace("default-workspace", &[project.clone()]);
    let session = session::start_session(project_id);
    let launch = runtime::launch_simulation(&project.manifest_hash, "everarcade-0.1");
    StudioApp { workspace_hash: workspace.workspace_hash, project_hash: project.manifest_hash, session_hash: session.session_hash, launch_hash: launch.launch_hash }
}
