use execution_core::treasury::{
    treasury::SovereignTreasury, treasury_transition::transition_treasury,
    treasury_validation::validate_treasury_replay,
};
#[test]
fn treasury_transition_replay_valid() {
    let t = SovereignTreasury {
        treasury_id: [1; 32],
        sovereign_domain: [2; 32],
        treasury_root: [0; 32],
        monetary_root: [3; 32],
        fiscal_root: [4; 32],
    };
    let n = transition_treasury(&t, [5; 32], [6; 32]);
    assert!(validate_treasury_replay(&n));
}
