use crate::world::persistence::append_checkpoint;
use crate::world::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PersistentWorldState {
    pub world_id: String,
    pub tick: u64,
    pub lifecycles: Vec<EntityLifecycle>,
    pub inventory_mutations: Vec<InventoryMutation>,
    pub economy_mutations: Vec<EconomyMutation>,
}

#[derive(Clone, Debug, Default)]
pub struct WorldSimulation {
    pub scheduler: WorldScheduler,
    pub archive: CivilizationArchive,
    pub state: PersistentWorldState,
}

impl WorldSimulation {
    pub fn tick(&mut self) -> WorldCheckpoint {
        let operations = self.scheduler.pop_tick(self.state.tick);
        for op in operations {
            let scenario = op.operation_id;
            self.state.economy_mutations.push(EconomyMutation {
                tick: self.state.tick,
                asset_id: format!("asset-{}", self.state.tick),
                from_owner: "vault".into(),
                to_owner: "entity-0".into(),
                reason: scenario.clone(),
            });
            self.state.inventory_mutations.push(InventoryMutation {
                tick: self.state.tick,
                owner_id: "entity-0".into(),
                asset_id: format!("asset-{}", self.state.tick),
                delta: 1,
                scenario,
            });
        }
        let checkpoint = WorldCheckpoint {
            tick: self.state.tick,
            continuity_root: WorldContinuityRoot(format!(
                "root:{}:{}:{}",
                self.state.tick,
                self.state.inventory_mutations.len(),
                self.state.economy_mutations.len()
            )),
            lifecycle: LifecycleCheckpoint {
                tick: self.state.tick,
                entity_count: self.state.lifecycles.len(),
            },
            ledger: EconomicLedgerCheckpoint {
                tick: self.state.tick,
                mutation_count: self.state.economy_mutations.len(),
                ledger_root: format!("ledger:{}", self.state.economy_mutations.len()),
            },
            scheduler: SchedulerCheckpoint {
                pending_tick_count: 0,
            },
        };
        append_checkpoint(&mut self.archive, checkpoint.clone());
        self.state.tick = self.state.tick.saturating_add(1);
        checkpoint
    }
}
