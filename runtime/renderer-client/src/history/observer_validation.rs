#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverValidation;

impl ObserverValidation {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
