pub mod cadence;
pub mod events;
pub mod executor;
pub mod queue;
pub mod tick;
pub mod window;
pub mod world;

use queue::DeterministicQueue;
use tick::DeterministicTick;
use world::{DeterministicWorld, TickReceipt};

use self::executor::DeterministicExecutor;

pub struct SchedulerRuntime<W>
where
    W: DeterministicWorld,
{
    queue: DeterministicQueue,
    executor: DeterministicExecutor<W>,
    tick: DeterministicTick,
}

impl<W> SchedulerRuntime<W>
where
    W: DeterministicWorld,
{
    pub fn new(world: W, queue: DeterministicQueue, start_tick: DeterministicTick) -> Self {
        Self {
            queue,
            executor: DeterministicExecutor::new(world),
            tick: start_tick,
        }
    }

    pub fn run_one_tick(&mut self) -> TickReceipt {
        let event = self.queue.pop_next();
        let receipt = self.executor.execute_tick(self.tick, event.as_ref());
        self.tick = self.tick.next();
        receipt
    }

    pub fn queue(&self) -> &DeterministicQueue {
        &self.queue
    }
}
