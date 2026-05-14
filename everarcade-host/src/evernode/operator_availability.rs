use super::capacity_manifest::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorAvailability {
    pub operator_id: Hash,
    pub available_windows: u64,
}
