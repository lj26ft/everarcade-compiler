use super::versions;

pub fn is_abi_compatible(version: &str) -> bool {
    version == versions::ABI_VERSION
}
pub fn is_receipt_compatible(version: &str) -> bool {
    version == versions::RECEIPT_VERSION
}
pub fn is_snapshot_compatible(version: &str) -> bool {
    version == versions::SNAPSHOT_VERSION
}

pub fn verifier_compatible(protocol_version: &str) -> bool {
    protocol_version == versions::PROTOCOL_VERSION
}
