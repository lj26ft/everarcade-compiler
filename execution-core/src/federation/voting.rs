#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vote { pub voter: String, pub support: bool }

pub fn tally(votes: &[Vote]) -> u64 { votes.iter().filter(|v| v.support).count() as u64 }
