#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationBoundary {
    pub contract_id: u64,
    pub memory_namespace: u64,
}

pub fn isolated(a: &IsolationBoundary, b: &IsolationBoundary) -> bool {
    a.memory_namespace != b.memory_namespace
}
