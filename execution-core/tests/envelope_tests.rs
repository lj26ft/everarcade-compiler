use execution_core::envelope::*;
use execution_core::federation::node::FederationNodeId;
use std::collections::{BTreeMap, BTreeSet};
fn node(v: u8) -> FederationNodeId {
    FederationNodeId([v; 32])
}
fn msg(v: u8) -> SignedContinuityMessage {
    let mut m = SignedContinuityMessage {
        message_id: [0; 32],
        sender: node(v),
        payload_hash: [v; 32],
    };
    m.message_id = hash_signed_message(&m);
    m
}
#[test]
fn test_envelope_descriptor_hash_stable() {
    let d = ContinuityEnvelopeDescriptor {
        envelope_id: [1; 32],
        message_hash: [2; 32],
        registry_hash: [3; 32],
    };
    assert_eq!(hash_envelope_descriptor(&d), hash_envelope_descriptor(&d));
}
#[test]
fn test_signed_message_valid() {
    assert!(verify_signed_message(&msg(1)).is_ok());
}
#[test]
fn test_duplicate_message_rejected() {
    let m = msg(1);
    let mut map = BTreeMap::new();
    map.insert(m.message_id, m.clone());
    let reg = EnvelopeRegistry {
        known_messages: map,
    };
    assert!(update_envelope_registry(&reg, m).is_err());
}
#[test]
fn test_signature_verification_valid() {
    let m = msg(2);
    let s = ContinuitySignature {
        signer: m.sender,
        signature_hash: [9; 32],
    };
    assert!(verify_continuity_signature(&s, &m).is_ok());
}
#[test]
fn test_registry_stable() {
    let r = EnvelopeRegistry {
        known_messages: BTreeMap::new(),
    };
    assert_eq!(hash_envelope_registry(&r), hash_envelope_registry(&r));
}
#[test]
fn test_envelope_verification_valid() {
    let m = msg(1);
    let mut ids = BTreeSet::new();
    ids.insert([42; 32]);
    let replay = ReplayProtection {
        known_message_ids: ids,
    };
    let reg = EnvelopeRegistry {
        known_messages: BTreeMap::new(),
    };
    let st = EnvelopeState {
        registry_hash: hash_envelope_registry(&reg),
        known_message_count: 0,
    };
    let report = verify_envelope(
        &m,
        &ContinuitySignature {
            signer: m.sender,
            signature_hash: [7; 32],
        },
        &reg,
        &replay,
        &EnvelopeQuarantine {
            quarantined_message: false,
        },
        &EnvelopePolicy {
            replay_protection_required: true,
        },
        &st,
    )
    .unwrap();
    assert!(report.valid && !report.replay_detected);
}
#[test]
fn test_replay_detection() {
    let m = msg(1);
    let mut ids = BTreeSet::new();
    ids.insert(m.message_id);
    assert!(verify_replay_protection(
        &ReplayProtection {
            known_message_ids: ids
        },
        &m.message_id
    )
    .is_err());
}
#[test]
fn test_envelope_quarantine_valid() {
    assert!(verify_envelope_quarantine(&EnvelopeQuarantine {
        quarantined_message: false
    })
    .is_ok());
}
#[test]
fn test_envelope_policy_requires_replay_protection() {
    assert!(verify_envelope_policy(
        &EnvelopePolicy {
            replay_protection_required: true
        },
        &ReplayProtection {
            known_message_ids: BTreeSet::new()
        }
    )
    .is_err());
}
#[test]
fn test_envelope_state_valid() {
    let reg = EnvelopeRegistry {
        known_messages: BTreeMap::new(),
    };
    let st = EnvelopeState {
        registry_hash: hash_envelope_registry(&reg),
        known_message_count: 0,
    };
    assert!(verify_envelope_state(&st, &reg).is_ok());
}
#[test]
fn test_envelope_replay_consistency() {
    let s = EnvelopeState {
        registry_hash: [1; 32],
        known_message_count: 1,
    };
    assert_eq!(hash_envelope_state(&s), hash_envelope_state(&s));
}
#[test]
fn test_envelope_deterministic() {
    assert_eq!(msg(3).message_id, msg(3).message_id);
}
