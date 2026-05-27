#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompressionTransport {
    pub deterministic: bool,
}
