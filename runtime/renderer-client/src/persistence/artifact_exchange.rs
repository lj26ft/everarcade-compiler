#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactExchange {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactExchange {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
