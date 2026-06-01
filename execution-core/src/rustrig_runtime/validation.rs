use contract_api::protocol_records::ProtocolRecord;

use super::error::{Result, RustrigRuntimeError};
use super::registry::RegisteredRustrig;

pub fn record_category(record: &ProtocolRecord) -> &'static str {
    match record {
        ProtocolRecord::World(_) => "world",
        ProtocolRecord::Entity(_) => "entity",
        ProtocolRecord::Combat(_) => "combat",
        ProtocolRecord::Inventory(_) => "inventory",
        ProtocolRecord::Quest(_) => "quest",
        ProtocolRecord::Dialogue(_) => "dialogue",
        ProtocolRecord::Economy(_) => "economy",
        ProtocolRecord::Ui(_) => "ui",
        ProtocolRecord::Replay(_) => "replay",
        ProtocolRecord::Deployment(_) => "deployment",
        ProtocolRecord::DeploymentIntent(_) => "deployment-intent",
        ProtocolRecord::Xrpl(_) => "xrpl",
        ProtocolRecord::XrplIntent(_) => "xrpl-intent",
    }
}

pub fn validate_output_abi(rig: &RegisteredRustrig, records: &[ProtocolRecord]) -> Result<()> {
    for record in records {
        let category = record_category(record);
        if !rig.allowed_categories.contains(&category) {
            return Err(RustrigRuntimeError::AbiIncompatible {
                rustrig_id: rig.id.to_string(),
                reason: format!("emitted disallowed category {category}"),
            });
        }
        if matches!(
            record,
            ProtocolRecord::Deployment(_) | ProtocolRecord::Xrpl(_)
        ) {
            return Err(RustrigRuntimeError::AuthorityMutationRejected(format!(
                "{} emitted authoritative external record {category}",
                rig.id
            )));
        }
    }
    Ok(())
}
