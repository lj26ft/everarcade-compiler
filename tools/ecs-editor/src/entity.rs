use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthoredEntity { pub id: String, pub lineage: String }

pub fn create_entity(id: &str, components: &[&str]) -> AuthoredEntity {
    let mut ordered = components.to_vec();
    ordered.sort_unstable();
    AuthoredEntity { id: id.to_owned(), lineage: stable_hash(&ordered) }
}
