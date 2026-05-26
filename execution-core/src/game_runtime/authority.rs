use super::entities::Entity;
pub fn authoritative(entity: &Entity, player_id: &str) -> bool { entity.authority == player_id }
