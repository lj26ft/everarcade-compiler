use serde::{Deserialize, Serialize};

use super::{environment::EnvironmentState, evolution, terrain::TerrainCell, validation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldSimulationError {
    NonDeterministicEvolution,
    InvalidMutation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSimulationRuntime {
    pub tick: u64,
    pub terrain: Vec<TerrainCell>,
    pub environment: EnvironmentState,
    pub replay_roots: Vec<String>,
}

impl Default for WorldSimulationRuntime {
    fn default() -> Self {
        Self {
            tick: 0,
            terrain: Vec::new(),
            environment: EnvironmentState {
                climate_index: 0,
                replay_root: String::new(),
            },
            replay_roots: Vec::new(),
        }
    }
}

impl WorldSimulationRuntime {
    pub fn add_cell(&mut self, cell: TerrainCell) {
        self.terrain.push(cell);
        self.terrain
            .sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    }
    pub fn evolve(&mut self, replay_root: &str) -> Result<(), WorldSimulationError> {
        if replay_root.is_empty() {
            return Err(WorldSimulationError::NonDeterministicEvolution);
        }
        for cell in &mut self.terrain {
            evolution::evolve_cell(cell, self.tick);
        }
        self.environment.climate_index += (self.tick as i64 % 5) - 2;
        self.environment.replay_root = replay_root.to_string();
        self.replay_roots.push(replay_root.to_string());
        if !validation::world_is_deterministic(self) {
            return Err(WorldSimulationError::InvalidMutation);
        }
        self.tick += 1;
        Ok(())
    }
}
