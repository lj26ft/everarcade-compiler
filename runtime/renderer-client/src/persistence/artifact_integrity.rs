#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactIntegrity {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactIntegrity {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
