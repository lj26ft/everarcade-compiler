#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub approve: bool,
}
