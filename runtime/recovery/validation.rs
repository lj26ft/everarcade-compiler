#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Validation;

impl Validation {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
