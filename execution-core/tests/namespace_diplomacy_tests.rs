use execution_core::namespace::namespace_resolution::resolve_namespace_conflict;
#[test]
fn namespace_conflict_has_stable_resolution() { assert_eq!(resolve_namespace_conflict([1;32],[2;32]), [1;32]); }
