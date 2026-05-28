use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProceduralWorldState {
    pub id: String,
    pub tick: u64,
    pub lineage: String,
    pub replay_tip: String,
    pub continuity_root: String,
    pub append_only_history: Vec<String>,
}

impl ProceduralWorldState {
    pub fn genesis(id: &str) -> Self {
        let lineage = format!("procedural_world:{id}:lineage:0");
        let replay_tip = format!("procedural_world:{id}:replay:0");
        let continuity_root = format!("procedural_world:{id}:continuity:0:{lineage}:{replay_tip}");
        Self {
            id: id.into(),
            tick: 0,
            lineage,
            replay_tip,
            continuity_root,
            append_only_history: vec![format!("procedural_world:{id}:genesis")],
        }
    }

    pub fn evolve(&mut self, input: &str) -> Result<(), &'static str> {
        let next = crate::procedural_world::evolution::evolve(self, input);
        crate::procedural_world::validation::validate(&next)?;
        *self = next;
        Ok(())
    }

    pub fn reject_authority_write(&self, replay_derived_write: bool) -> Result<(), &'static str> {
        if replay_derived_write {
            Err("replay-derived authority mutation rejected")
        } else {
            Ok(())
        }
    }
}
