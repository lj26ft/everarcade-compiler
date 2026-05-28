#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityPriority {
    pub priority: u8,
    pub entity_id: String,
}
