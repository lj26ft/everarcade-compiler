#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumRule { pub numerator: u64, pub denominator: u64 }

impl QuorumRule {
    pub fn reached(&self, votes: u64, members: u64) -> bool {
        if members == 0 || self.denominator == 0 { return false; }
        votes * self.denominator >= members * self.numerator
    }
}
