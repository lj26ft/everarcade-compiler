use super::{
    events::ScheduledEvent,
    tick::DeterministicTick,
    world::{DeterministicWorld, TickReceipt, WorldCheckpoint},
};

pub struct DeterministicExecutor<W>
where
    W: DeterministicWorld,
{
    world: W,
}

impl<W> DeterministicExecutor<W>
where
    W: DeterministicWorld,
{
    pub fn new(world: W) -> Self {
        Self { world }
    }

    pub fn execute_tick(
        &mut self,
        tick: DeterministicTick,
        event: Option<&ScheduledEvent>,
    ) -> TickReceipt {
        let receipt = self.world.apply(tick, event);
        let checkpoint = WorldCheckpoint {
            lineage: receipt.lineage,
            tick,
        };
        self.world.persist_checkpoint(checkpoint);
        receipt
    }

    pub fn world(&self) -> &W {
        &self.world
    }
}
