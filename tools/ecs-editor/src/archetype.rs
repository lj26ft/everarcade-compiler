use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchetypeView { pub components: Vec<String>, pub archetype_hash: String }

pub fn visualize_archetype(components: &[&str]) -> ArchetypeView {
    let mut ordered = components.to_vec();
    ordered.sort_unstable();
    ArchetypeView { components: ordered.iter().map(|c| (*c).to_owned()).collect(), archetype_hash: stable_hash(&ordered) }
}
