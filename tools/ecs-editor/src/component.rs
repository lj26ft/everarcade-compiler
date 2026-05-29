use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentSpec { pub name: String, pub schema_hash: String, pub deterministic: bool }

pub fn author_component(name: &str, fields: &[&str]) -> ComponentSpec {
    let mut ordered = fields.to_vec();
    ordered.sort_unstable();
    ComponentSpec { name: name.to_owned(), schema_hash: stable_hash(&ordered), deterministic: true }
}
