#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalSchema {
    pub name: &'static str,
    pub version: u16,
}
