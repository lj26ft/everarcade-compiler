use crate::studio::project::StudioProject;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioWorkspace {
    pub workspace_id: String,
    pub project_ids: Vec<String>,
    pub workspace_hash: String,
}

pub fn open_workspace(workspace_id: &str, projects: &[StudioProject]) -> StudioWorkspace {
    let mut project_ids: Vec<String> = projects.iter().map(|p| p.project_id.clone()).collect();
    project_ids.sort();
    let mut parts = vec!["studio-workspace", workspace_id];
    parts.extend(project_ids.iter().map(String::as_str));
    StudioWorkspace { workspace_id: workspace_id.to_owned(), workspace_hash: crate::stable_hash(&parts), project_ids }
}
