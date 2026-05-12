use std::collections::BTreeSet;

pub type Hash = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionNode {
    pub node_id: Hash,
    pub dependencies: BTreeSet<Hash>,
    pub execution_payload: ExecutionPayload,
    pub execution_policy: ExecutionPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionPayload {
    pub kind: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionPolicy {
    Required,
    Optional,
    Retryable,
    Compensating,
    ForkOnFailure,
}
