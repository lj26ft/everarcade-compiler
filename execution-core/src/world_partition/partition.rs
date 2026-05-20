use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{
    continuity::PartitionContinuity, error::PartitionError, migration::MigrationRecord,
    ownership::RegionOwnership, region::RegionState,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WorldPartition {
    pub partition_id: String,
    pub tick: u64,
    pub regions: BTreeMap<String, RegionState>,
    pub ownership: BTreeMap<String, RegionOwnership>,
    pub entity_regions: BTreeMap<String, String>,
    pub continuity: PartitionContinuity,
    pub event_sequences: Vec<u64>,
}

impl WorldPartition {
    pub fn assign_region_owner(
        &mut self,
        region_id: &str,
        owner_node: &str,
        continuity_root: &str,
    ) {
        let epoch = self
            .ownership
            .get(region_id)
            .map(|o| o.epoch + 1)
            .unwrap_or(0);
        let ownership = RegionOwnership {
            region_id: region_id.to_string(),
            owner_node: owner_node.to_string(),
            epoch,
            continuity_root: continuity_root.to_string(),
        };
        self.ownership
            .insert(region_id.to_string(), ownership.clone());
        self.continuity.ownership_history.push(ownership);
    }

    pub fn transfer_region_ownership(
        &mut self,
        region_id: &str,
        new_owner: &str,
    ) -> Result<(), PartitionError> {
        let old = self
            .ownership
            .get(region_id)
            .cloned()
            .ok_or_else(|| PartitionError::RegionNotFound(region_id.to_string()))?;
        self.assign_region_owner(region_id, new_owner, &old.continuity_root);
        Ok(())
    }

    pub fn migrate_entity_partition(
        &mut self,
        entity_id: &str,
        target_region: &str,
    ) -> Result<MigrationRecord, PartitionError> {
        let source_region = self
            .entity_regions
            .get(entity_id)
            .cloned()
            .ok_or_else(|| PartitionError::InvalidMigration(entity_id.to_string()))?;
        if source_region == target_region {
            return Err(PartitionError::InvalidMigration(entity_id.to_string()));
        }
        let epoch = self
            .ownership
            .get(target_region)
            .map(|o| o.epoch)
            .unwrap_or(0);
        let record = MigrationRecord {
            entity_id: entity_id.to_string(),
            source_region,
            target_region: target_region.to_string(),
            ownership_epoch: epoch,
            continuity_proof: format!("{}:{}:{}", self.partition_id, entity_id, target_region),
            sequence: self.continuity.migrations.len() as u64,
        };
        self.entity_regions
            .insert(entity_id.to_string(), target_region.to_string());
        self.continuity.migrations.push(record.clone());
        Ok(record)
    }

    pub fn resume_partition_execution(&mut self) {
        self.tick += 1;
    }

    pub fn propagate_partition_event(&mut self, sequence: u64) {
        self.event_sequences.push(sequence);
    }

    pub fn resolve_cross_partition_interaction(&mut self, interaction_id: &str) {
        self.continuity.lineage.push(interaction_id.to_string());
    }
}
