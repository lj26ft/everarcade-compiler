use everarcade_abi::ExecutionNode;

use super::registry::ContractRegistry;

pub fn resolve_contract_path<'a>(registry: &'a ContractRegistry, node: &ExecutionNode) -> Option<&'a str> {
    registry.resolve(&node.contract_id)
}
