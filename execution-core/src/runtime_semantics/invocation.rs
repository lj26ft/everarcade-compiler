#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub contract_id: u64,
    pub depth: u8,
}

pub fn canonical_invocation_order(mut invocations: Vec<Invocation>) -> Vec<Invocation> {
    invocations.sort_by_key(|i| (i.depth, i.contract_id));
    invocations
}
