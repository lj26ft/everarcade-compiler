use super::{compatibility, versions};

pub fn validate_protocol_upgrade(target_protocol_version: &str) -> Result<(), String> {
    if compatibility::verifier_compatible(target_protocol_version) {
        return Ok(());
    }

    Err(format!(
        "incompatible protocol version: {target_protocol_version}; current frozen version is {}",
        versions::PROTOCOL_VERSION
    ))
}
