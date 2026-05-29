use crate::studio::{project, runtime, session, workspace};

pub fn validate_studio_surface() -> crate::CreatorDiagnostic {
    crate::diagnostic("everarcade-studio", &["workspace", "projects", "runtime", "replay", "packages", "deployment"])
}

pub fn studio_workflow_equivalence(project_id: &str) -> bool {
    let a = session::start_session(project_id);
    let b = session::start_session(project_id);
    a == b && a.replay_safe && a.workflow == session::CREATOR_WORKFLOW
}

pub fn studio_launches() -> bool {
    let p = project::create_project("project", "seed");
    let w = workspace::open_workspace("workspace", &[p.clone()]);
    let launch = runtime::launch_simulation(&p.manifest_hash, "everarcade-0.1");
    !w.project_ids.is_empty() && launch.projection_only_viewport && launch.deterministic_runtime_authority
}
