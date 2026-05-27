#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactScheduler {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactScheduler {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
