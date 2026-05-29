use crate::{layout::WorkspaceLayout, stable_hash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioProject {
    pub project_id: String,
    pub manifest_hash: String,
    pub validation_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSessionView {
    pub session_id: String,
    pub project_id: String,
    pub projection_root: String,
    pub replay_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioWorkspace {
    pub workspace_id: String,
    pub projects: Vec<StudioProject>,
    pub layout: WorkspaceLayout,
    pub runtime_sessions: Vec<RuntimeSessionView>,
    pub persisted_layout: String,
}

impl StudioWorkspace {
    pub fn new(workspace_id: &str) -> Self {
        let project = create_project("starter-world", "template:simulation-world");
        let layout = WorkspaceLayout::default_creator_layout();
        let persisted_layout = layout.serialize();
        let runtime_sessions = vec![open_runtime_session(&project.project_id, "session-1")];
        Self {
            workspace_id: workspace_id.to_owned(),
            projects: vec![project],
            layout,
            runtime_sessions,
            persisted_layout,
        }
    }

    pub fn save_layout(&mut self) {
        self.persisted_layout = self.layout.serialize();
    }

    pub fn restore_layout(&mut self) {
        self.layout = WorkspaceLayout::restore(&self.persisted_layout);
    }

    pub fn workspace_hash(&self) -> String {
        let mut parts = vec![
            "studio-gui-workspace",
            &self.workspace_id,
            &self.layout.deterministic_state_hash,
        ];
        parts.extend(
            self.projects
                .iter()
                .map(|project| project.manifest_hash.as_str()),
        );
        stable_hash(&parts)
    }

    pub fn supports_multiple_projects(&self) -> bool {
        let a = create_project("project-a", "seed");
        let b = create_project("project-b", "seed");
        a.manifest_hash != b.manifest_hash
    }
}

pub fn create_project(project_id: &str, template: &str) -> StudioProject {
    StudioProject {
        project_id: project_id.to_owned(),
        manifest_hash: stable_hash(&["project", project_id, template]),
        validation_state: "deterministic-editor-actions-only".to_owned(),
    }
}

pub fn open_project(project_id: &str) -> StudioProject {
    create_project(project_id, "opened")
}

pub fn clone_project(source: &StudioProject, clone_id: &str) -> StudioProject {
    create_project(clone_id, &source.manifest_hash)
}

pub fn import_template(template_id: &str) -> String {
    stable_hash(&["template-import", template_id])
}

pub fn validate_project(project: &StudioProject) -> bool {
    project.validation_state == "deterministic-editor-actions-only"
        && !project.manifest_hash.is_empty()
}

pub fn open_runtime_session(project_id: &str, session_id: &str) -> RuntimeSessionView {
    RuntimeSessionView {
        session_id: session_id.to_owned(),
        project_id: project_id.to_owned(),
        projection_root: stable_hash(&["runtime-projection", project_id, session_id]),
        replay_root: stable_hash(&["replay-root", project_id, session_id]),
    }
}
