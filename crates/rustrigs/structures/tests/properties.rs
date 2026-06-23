use everarcade_rustrig_structures::*;

#[test]
fn candidate_status_is_exposed() {
    assert_eq!(certified_status(), "CANDIDATE");
}
