#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mutation {
    pub key: String,
    pub value: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionPayload {
    Federation(Vec<Mutation>),
    Entity(Vec<Mutation>),
    Treaty(Vec<Mutation>),
    Governance(Vec<Mutation>),
    Vault(Vec<Mutation>),
    Wasm(Vec<Mutation>),
}

impl ExecutionPayload {
    pub fn mutations(&self) -> &[Mutation] {
        match self {
            Self::Federation(v)
            | Self::Entity(v)
            | Self::Treaty(v)
            | Self::Governance(v)
            | Self::Vault(v)
            | Self::Wasm(v) => v,
        }
    }
}
