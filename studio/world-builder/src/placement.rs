#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placement { pub entity_id: String, pub archetype: String, pub x: i32, pub y: i32 }
pub fn placement_manifest(placements: &[Placement]) -> String { let mut rows: Vec<String> = placements.iter().map(|p| format!("{}:{}:{}:{}", p.entity_id, p.archetype, p.x, p.y)).collect(); rows.sort(); let mut parts = vec!["placements"]; parts.extend(rows.iter().map(String::as_str)); crate::stable_hash(&parts) }
pub fn reject_hidden_state_mutation(requested: bool) -> Result<(), &'static str> { if requested { Err("world builder cannot commit hidden state mutation") } else { Ok(()) } }
