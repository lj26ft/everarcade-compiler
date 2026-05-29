use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorProject { pub project_id: String, pub lineage: String }

pub fn manage_project(project_id: &str) -> CreatorProject { CreatorProject { project_id: project_id.to_owned(), lineage: stable_hash(&["project", project_id]) } }
