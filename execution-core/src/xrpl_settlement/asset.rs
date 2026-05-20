use serde::{Deserialize, Serialize};

use super::{error::SettlementError, ownership::AssetOwnership};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AssetLineage {
    pub ownership: Vec<AssetOwnership>,
    pub continuity_root: [u8; 32],
}

pub fn assign_asset_owner(
    lineage: &mut AssetLineage,
    asset_id: &str,
    owner_id: &str,
    epoch: u64,
) -> Result<(), SettlementError> {
    if lineage.ownership.iter().any(|o| o.asset_id == asset_id) {
        return Err(SettlementError::AssetExists(asset_id.to_string()));
    }
    lineage.ownership.push(AssetOwnership {
        asset_id: asset_id.into(),
        owner_id: owner_id.into(),
        settlement_epoch: epoch,
    });
    lineage.ownership.sort_by(|a, b| {
        a.asset_id
            .cmp(&b.asset_id)
            .then(a.settlement_epoch.cmp(&b.settlement_epoch))
            .then(a.owner_id.cmp(&b.owner_id))
    });
    Ok(())
}

pub fn transfer_asset_ownership(
    lineage: &mut AssetLineage,
    asset_id: &str,
    from: &str,
    to: &str,
    epoch: u64,
) -> Result<(), SettlementError> {
    let current = lineage
        .ownership
        .iter()
        .rev()
        .find(|o| o.asset_id == asset_id)
        .cloned()
        .ok_or_else(|| SettlementError::AssetMissing(asset_id.into()))?;
    if current.owner_id != from {
        return Err(SettlementError::OwnershipMismatch {
            asset_id: asset_id.into(),
            expected: from.into(),
            actual: current.owner_id,
        });
    }
    lineage.ownership.push(AssetOwnership {
        asset_id: asset_id.into(),
        owner_id: to.into(),
        settlement_epoch: epoch,
    });
    lineage.ownership.sort_by(|a, b| {
        a.asset_id
            .cmp(&b.asset_id)
            .then(a.settlement_epoch.cmp(&b.settlement_epoch))
            .then(a.owner_id.cmp(&b.owner_id))
    });
    Ok(())
}

pub fn verify_asset_lineage(lineage: &AssetLineage) -> bool {
    lineage
        .ownership
        .windows(2)
        .all(|w| w[0].asset_id <= w[1].asset_id)
}
