use super::capability::Capability;

pub fn revoke(capability: &Capability, revocation_root: [u8; 32]) -> Capability {
    let mut updated = capability.clone();
    updated.revocation_root = Some(revocation_root);
    updated
}
