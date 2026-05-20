use super::{error::PartitionError, migration::MigrationRecord, ownership::RegionOwnership};

pub fn verify_region_continuity(
    ownership: &RegionOwnership,
    expected_owner: &str,
) -> Result<(), PartitionError> {
    if ownership.owner_node == expected_owner {
        Ok(())
    } else {
        Err(PartitionError::OwnershipMismatch(
            ownership.region_id.clone(),
        ))
    }
}

pub fn verify_partition_migration(record: &MigrationRecord) -> Result<(), PartitionError> {
    if record.source_region == record.target_region || record.continuity_proof.is_empty() {
        return Err(PartitionError::InvalidMigration(record.entity_id.clone()));
    }
    Ok(())
}

pub fn verify_partition_interaction(sequence: &[u64]) -> bool {
    sequence.windows(2).all(|w| w[0] <= w[1])
}
