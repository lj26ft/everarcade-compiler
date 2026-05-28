#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverValidation;

impl ObserverValidation {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}

pub fn reject_observer_authority_mutation(reconstruction_only: bool) -> Result<(), String> {
    if reconstruction_only {
        Ok(())
    } else {
        Err("observer_mutation_rejected".into())
    }
}
