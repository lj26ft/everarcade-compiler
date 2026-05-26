#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeStressValidation {
    pub deterministic_ordering: bool,
    pub replay_equivalence: bool,
    pub dag_determinism: bool,
    pub restoration_equivalence: bool,
}

impl RuntimeStressValidation {
    pub fn is_stable(&self) -> bool {
        self.deterministic_ordering
            && self.replay_equivalence
            && self.dag_determinism
            && self.restoration_equivalence
    }
}
