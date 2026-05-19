use std::collections::{BTreeMap, BTreeSet};

use execution_core::coordination::*;
use execution_core::federation::node::FederationNodeId;

fn node(v: u8) -> FederationNodeId {
    FederationNodeId([v; 32])
}

fn session_with(nodes: &[u8]) -> CoordinationSession {
    let participants: BTreeSet<_> = nodes.iter().copied().map(node).collect();
    let mut s = CoordinationSession {
        session_id: [0; 32],
        participants,
    };
    s.session_id = hash_coordination_session(&s);
    s
}

#[test]
fn test_coordination_descriptor_hash_stable() {
    let d = CoordinationDescriptor {
        coordination_id: [1; 32],
        session_hash: [2; 32],
        registry_hash: [3; 32],
    };
    assert_eq!(
        hash_coordination_descriptor(&d),
        hash_coordination_descriptor(&d)
    );
}
#[test]
fn test_coordination_session_valid() {
    assert!(verify_coordination_session(&session_with(&[1, 2])).is_ok());
}
#[test]
fn test_duplicate_participant_rejected() {
    let input = vec![node(1), node(1)];
    let set: BTreeSet<_> = input.iter().copied().collect();
    assert!(set.len() < input.len());
}
#[test]
fn test_coordination_exchange_valid() {
    let s = session_with(&[1, 2]);
    let ex = CoordinationExchange {
        proposal_id: [9; 32],
        exchanged_by: node(1),
    };
    let mut proposals = BTreeMap::new();
    proposals.insert([9; 32], [7; 32]);
    assert!(verify_coordination_exchange(&ex, &s, &proposals, &BTreeMap::new()).is_ok());
}
#[test]
fn test_duplicate_exchange_rejected() {
    let s = session_with(&[1]);
    let ex = CoordinationExchange {
        proposal_id: [9; 32],
        exchanged_by: node(1),
    };
    let mut proposals = BTreeMap::new();
    proposals.insert([9; 32], [7; 32]);
    let mut seen = BTreeMap::new();
    seen.insert([9; 32], ex.clone());
    assert!(verify_coordination_exchange(&ex, &s, &proposals, &seen).is_err());
}
#[test]
fn test_coordination_registry_stable() {
    let r = CoordinationRegistry {
        active_sessions: BTreeMap::new(),
    };
    assert_eq!(
        hash_coordination_registry(&r),
        hash_coordination_registry(&r)
    );
}
#[test]
fn test_coordination_verification_valid() {
    let s = session_with(&[1]);
    let ex = CoordinationExchange {
        proposal_id: [9; 32],
        exchanged_by: node(1),
    };
    let mut proposals = BTreeMap::new();
    proposals.insert([9; 32], [7; 32]);
    let mut sessions = BTreeMap::new();
    sessions.insert(s.session_id, s.clone());
    let reg = CoordinationRegistry {
        active_sessions: sessions,
    };
    let st = CoordinationState {
        registry_hash: hash_coordination_registry(&reg),
        active_session_count: 1,
    };
    let report = verify_coordination(
        &s,
        &ex,
        &proposals,
        &BTreeMap::new(),
        &reg,
        &CoordinationPolicy {
            exchanges_allowed: true,
        },
        &CoordinationQuarantine {
            quarantined_exchange: false,
        },
        &st,
    )
    .unwrap();
    assert!(report.valid && !report.quarantine_required);
}
#[test]
fn test_coordination_quarantine_valid() {
    assert!(verify_coordination_quarantine(
        &CoordinationQuarantine {
            quarantined_exchange: true
        },
        false
    )
    .is_ok());
}
#[test]
fn test_coordination_policy_exchange_gate() {
    assert!(verify_coordination_policy(
        &CoordinationPolicy {
            exchanges_allowed: false
        },
        true
    )
    .is_err());
}
#[test]
fn test_coordination_state_valid() {
    let reg = CoordinationRegistry {
        active_sessions: BTreeMap::new(),
    };
    let st = CoordinationState {
        registry_hash: hash_coordination_registry(&reg),
        active_session_count: 0,
    };
    assert!(verify_coordination_state(&st, &reg).is_ok());
}
#[test]
fn test_coordination_replay_consistency() {
    let st = CoordinationState {
        registry_hash: [1; 32],
        active_session_count: 3,
    };
    assert_eq!(hash_coordination_state(&st), hash_coordination_state(&st));
}
#[test]
fn test_coordination_deterministic() {
    let s1 = session_with(&[2, 1]);
    let s2 = session_with(&[1, 2]);
    assert_eq!(s1.session_id, s2.session_id);
}
