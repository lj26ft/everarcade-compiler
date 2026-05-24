use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Spawn,
    Mutation,
    Migration,
    Upgrade,
    Archival,
    Restoration,
    Retirement,
}
