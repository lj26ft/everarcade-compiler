use crate::{creator_dashboard, diagnostic, CreatorDiagnostic};

pub fn validate_creator_dashboard() -> CreatorDiagnostic { diagnostic("creator-dashboard-validation", &["projects", "deployment", "replay", "packages"] ) }

pub fn creator_dashboard_equivalence(project_id: &str) -> bool { creator_dashboard::projects::manage_project(project_id) == creator_dashboard::projects::manage_project(project_id) }

pub fn reject_invalid_package_mutation(mutate: bool) -> Result<(), &'static str> { if mutate { Err("creator dashboard cannot mutate deterministic packages outside validation") } else { Ok(()) } }
