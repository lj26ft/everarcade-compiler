#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactStream {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactStream {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
