use super::{cadence::TickCadence, tick::DeterministicTick};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TickWindow {
    pub start: DeterministicTick,
    pub end: DeterministicTick,
}

impl TickWindow {
    pub fn from_tick(cadence: TickCadence, tick: DeterministicTick) -> Self {
        let size = cadence.ticks_per_window.max(1);
        let start_n = (tick.0 / size) * size;
        let end_n = start_n + (size - 1);
        Self {
            start: DeterministicTick(start_n),
            end: DeterministicTick(end_n),
        }
    }
}
