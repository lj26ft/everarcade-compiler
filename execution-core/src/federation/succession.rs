#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuccessionRecord {
    pub predecessor: String,
    pub successor: String,
    pub continuity_root: String,
}
