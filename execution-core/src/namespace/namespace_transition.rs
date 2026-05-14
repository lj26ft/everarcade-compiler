use super::namespace::ExecutionNamespace;

pub fn transfer_namespace(
    namespace: &ExecutionNamespace,
    owning_domain: [u8; 32],
) -> ExecutionNamespace {
    let mut next = namespace.clone();
    next.owning_domain = owning_domain;
    next
}
