use execution_core::{
    authority::{
        hash_authority_descriptor, hash_authority_epoch, hash_authority_handoff,
        update_authority_registry, verify_authority_chain, verify_epoch_transition,
        verify_execution_authority, verify_execution_policy, verify_handoff,
        verify_rotation_policy, AuthorityDescriptor, AuthorityEpoch, AuthorityHandoff,
        AuthorityRegistry, ExecutionAuthorityProof, ExecutionPolicy, RotationPolicy,
    },
    federation::node::FederationNodeId,
};

fn n(v: u8) -> FederationNodeId {
    FederationNodeId::new([v; 32])
}

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

fn epoch0() -> AuthorityEpoch {
    AuthorityEpoch {
        epoch: 0,
        authority: n(1),
        previous_epoch_hash: None,
    }
}

fn epoch1() -> AuthorityEpoch {
    let previous = epoch0();
    AuthorityEpoch {
        epoch: 1,
        authority: n(2),
        previous_epoch_hash: Some(hash_authority_epoch(&previous)),
    }
}

fn handoff1() -> AuthorityHandoff {
    AuthorityHandoff {
        from: n(1),
        to: n(2),
        epoch: 1,
        checkpoint_root: h(9),
        lineage_hash: h(8),
    }
}

#[test]
fn test_authority_descriptor_hash_stable() {
    let descriptor = AuthorityDescriptor {
        authority_node: n(1),
        epoch: 7,
        package_root: h(2),
        checkpoint_root: h(3),
    };
    assert_eq!(
        hash_authority_descriptor(&descriptor),
        hash_authority_descriptor(&descriptor)
    );
}

#[test]
fn test_epoch_monotonic() {
    assert!(verify_epoch_transition(&epoch0(), &epoch1()).is_ok());
}

#[test]
fn test_epoch_rollback_rejected() {
    assert!(verify_epoch_transition(&epoch1(), &epoch0()).is_err());
}

#[test]
fn test_handoff_valid() {
    assert!(verify_handoff(&epoch0(), &epoch1(), &handoff1(), h(9), h(8)).is_ok());
}

#[test]
fn test_handoff_checkpoint_mismatch_fails() {
    assert!(verify_handoff(&epoch0(), &epoch1(), &handoff1(), h(7), h(8)).is_err());
}

#[test]
fn test_handoff_lineage_mismatch_fails() {
    assert!(verify_handoff(&epoch0(), &epoch1(), &handoff1(), h(9), h(7)).is_err());
}

#[test]
fn test_execution_authority_valid() {
    let registry = AuthorityRegistry {
        current_authority: n(2),
        current_epoch: 1,
    };
    let proof = ExecutionAuthorityProof {
        authority: n(2),
        epoch: 1,
        execution_id: h(4),
    };
    assert!(verify_execution_authority(&proof, &registry).is_ok());
}

#[test]
fn test_unauthorized_execution_rejected() {
    let registry = AuthorityRegistry {
        current_authority: n(2),
        current_epoch: 1,
    };
    let proof = ExecutionAuthorityProof {
        authority: n(3),
        epoch: 1,
        execution_id: h(4),
    };
    assert!(verify_execution_authority(&proof, &registry).is_err());
}

#[test]
fn test_authority_chain_valid() {
    let report = verify_authority_chain(&[epoch0(), epoch1()], &[handoff1()]);
    assert!(report.authorized);
    assert!(report.epoch_valid);
    assert!(report.handoff_valid);
}

#[test]
fn test_authority_chain_divergence_detected() {
    let mut handoff = handoff1();
    handoff.to = n(3);
    let report = verify_authority_chain(&[epoch0(), epoch1()], &[handoff]);
    assert!(!report.authorized);
    assert!(report.epoch_valid);
    assert!(!report.handoff_valid);
}

#[test]
fn test_registry_monotonic() {
    let registry = AuthorityRegistry {
        current_authority: n(1),
        current_epoch: 0,
    };
    let updated = update_authority_registry(&registry, &epoch1()).unwrap();
    assert_eq!(updated.current_authority, n(2));
    assert_eq!(updated.current_epoch, 1);
    assert!(update_authority_registry(&updated, &epoch0()).is_err());
}

#[test]
fn test_rotation_policy_valid() {
    assert!(verify_rotation_policy(
        &RotationPolicy {
            allow_handoff: true
        },
        &handoff1()
    )
    .is_ok());
    assert!(verify_rotation_policy(
        &RotationPolicy {
            allow_handoff: false
        },
        &handoff1()
    )
    .is_err());
}

#[test]
fn test_execution_policy_single_authority() {
    let policy = ExecutionPolicy {
        single_authority_required: true,
    };
    assert!(verify_execution_policy(&policy, &[n(1), n(1)]).is_ok());
    assert!(verify_execution_policy(&policy, &[n(1), n(2)]).is_err());
}

#[test]
fn test_authority_replay_consistency() {
    let epochs = vec![epoch0(), epoch1()];
    let handoffs = vec![handoff1()];
    assert_eq!(
        verify_authority_chain(&epochs, &handoffs),
        verify_authority_chain(&epochs, &handoffs)
    );
}

#[test]
fn test_authority_handoff_deterministic() {
    let handoff = handoff1();
    assert_eq!(
        hash_authority_handoff(&handoff),
        hash_authority_handoff(&handoff)
    );
}
