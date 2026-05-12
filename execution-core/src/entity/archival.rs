use super::identity::EntityIdentity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchivedEntity {
    pub identity: EntityIdentity,
    pub archive_ref: String,
}
