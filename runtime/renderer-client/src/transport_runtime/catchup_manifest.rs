#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CatchupManifest {
    pub deterministic: bool,
}
