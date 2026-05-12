#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuelMeter { pub remaining: u64 }

impl FuelMeter {
    pub fn new(limit: u64) -> Self { Self { remaining: limit } }
    pub fn charge(&mut self, units: u64) -> bool {
        if self.remaining < units { return false; }
        self.remaining -= units;
        true
    }
}
