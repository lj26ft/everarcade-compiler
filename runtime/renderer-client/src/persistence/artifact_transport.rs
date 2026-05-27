#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactTransport {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactTransport {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
