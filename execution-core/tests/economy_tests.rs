use execution_core::economy::{
    archival_market::archival_reward,
    continuity::continuity_score,
    epoch_economics::transition_multiplier,
    execution_market::{match_offer, ExecutionBid, ExecutionOffer},
    pricing::PricingInput,
    proof_market::proof_reward,
    slashing::slash_amount,
    sovereignty::sovereignty_index,
    verifier_rewards::verifier_reward,
};

#[test]
fn test_execution_market_determinism() {
    let offers = vec![
        ExecutionOffer { executor_id: "b".into(), capacity: 1_000, latency_hint_ms: 15 },
        ExecutionOffer { executor_id: "a".into(), capacity: 1_000, latency_hint_ms: 15 },
    ];
    let bid = ExecutionBid { workload: PricingInput { fuel_units: 500, proof_units: 10, archival_bytes: 4096, routing_packets: 12, settlement_ops: 3 } };
    let (winner, pricing) = match_offer(&offers, &bid).unwrap();
    assert_eq!(winner.executor_id, "a");
    assert_eq!(pricing.total, 1143);
}

#[test]
fn test_verifier_reward_consistency() {
    let a = verifier_reward(10, 2, 5);
    let b = verifier_reward(10, 2, 5);
    assert_eq!(a, b);
}

#[test]
fn test_archival_continuity() {
    assert_eq!(archival_reward(10_240), 11);
}

#[test]
fn test_slashing_consistency() {
    assert_eq!(slash_amount(1_000, 3), 300);
}

#[test]
fn test_epoch_economic_transition() {
    assert_eq!(transition_multiplier(8, 12), 104);
}

#[test]
fn test_sovereignty_preservation() {
    let before = sovereignty_index(2, 20, 50);
    let after = sovereignty_index(2, 20, 50);
    assert_eq!(before, after);
}

#[test]
fn test_proof_market_determinism() {
    assert_eq!(proof_reward(100, 2), proof_reward(100, 2));
}

#[test]
fn test_continuity_guarantees() {
    let score = continuity_score(5, 5, 2, 10);
    assert!(score >= 170);
}
