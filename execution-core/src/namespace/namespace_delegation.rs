use super::namespace::ExecutionNamespace;

pub fn delegate_namespace(
    namespace: &ExecutionNamespace,
    delegation_root: [u8; 32],
) -> ExecutionNamespace {
    let mut next = namespace.clone();
    next.delegation_root = delegation_root;
    next
}
