#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeterministicTick(pub u64);

impl DeterministicTick {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}
