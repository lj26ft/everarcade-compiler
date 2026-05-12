use super::namespace::{ExecutionNamespace, Hash};

pub fn namespace_root(namespace: &ExecutionNamespace) -> Hash {
    namespace.namespace_id
}
