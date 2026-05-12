use super::namespace::ExecutionNamespace;

pub fn validate_namespace(namespace: &ExecutionNamespace) -> bool {
    namespace.namespace_id != [0; 32]
        && namespace.owning_domain != [0; 32]
        && namespace.delegation_root != [0; 32]
}
