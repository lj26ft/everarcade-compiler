use super::domain::ExecutionDomain;

pub fn is_lineage_child(child: &ExecutionDomain, parent: &ExecutionDomain) -> bool {
    child.parent_domain == Some(parent.domain_id)
}
