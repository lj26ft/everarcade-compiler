use super::tick::DeterministicTick;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TickCadence {
    pub ticks_per_window: u64,
}

impl TickCadence {
    pub fn validate(self) -> Result<Self, &'static str> {
        if self.ticks_per_window == 0 {
            return Err("ticks_per_window must be greater than zero");
        }
        Ok(self)
    }

    pub fn in_same_window(self, a: DeterministicTick, b: DeterministicTick) -> bool {
        let ticks = self.ticks_per_window.max(1);
        (a.0 / ticks) == (b.0 / ticks)
    }
}
