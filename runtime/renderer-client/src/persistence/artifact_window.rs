#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactWindow {
    pub append_only: bool,
    pub resumable: bool,
}

impl ArtifactWindow {
    pub fn valid(&self) -> bool {
        self.append_only && self.resumable
    }
}
