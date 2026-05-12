use super::federation::Federation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationIdentity {
    pub federation_id: String,
    pub continuity_root: String,
}

impl From<&Federation> for FederationIdentity {
    fn from(value: &Federation) -> Self {
        Self { federation_id: value.federation_id.clone(), continuity_root: value.continuity_root.clone() }
    }
}
