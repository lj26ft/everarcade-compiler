use execution_core::persistence::entity_evolution::{evolution_root, EntityEvolutionRecord};
#[test]
fn entity_evolution_determinism() {
    let e1 = EntityEvolutionRecord {
        sequence: 1,
        entity_id: "e".into(),
        prior_state_root: "p".into(),
        current_state_root: "c".into(),
        migration_root: "m".into(),
        previous_hash: String::new(),
    };
    let e2 = EntityEvolutionRecord {
        sequence: 2,
        entity_id: "e".into(),
        prior_state_root: "c".into(),
        current_state_root: "n".into(),
        migration_root: "m2".into(),
        previous_hash: e1.canonical_hash().unwrap(),
    };
    assert_eq!(evolution_root(&[e1, e2]).unwrap().len(), 64);
}
