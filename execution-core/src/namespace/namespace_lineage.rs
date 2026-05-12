use super::namespace::ExecutionNamespace;

pub fn same_owner(left: &ExecutionNamespace, right: &ExecutionNamespace) -> bool {
    left.owning_domain == right.owning_domain
}
