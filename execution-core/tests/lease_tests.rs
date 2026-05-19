use execution_core::{federation::node::FederationNodeId, leases::*};

fn lease(epoch: u64, start: u64, end: u64) -> ExecutionLease {
    ExecutionLease {
        authority: FederationNodeId::new([9u8; 32]),
        epoch,
        lease_start_tick: start,
        lease_end_tick: end,
        checkpoint_root: [7u8; 32],
    }
}

#[test]
fn test_lease_hash_stable() {
    let l = lease(1, 10, 20);
    assert_eq!(hash_execution_lease(&l), hash_execution_lease(&l));
}
#[test]
fn test_lease_window_valid() {
    assert!(verify_lease_window(
        &LeaseWindow {
            start_tick: 1,
            end_tick: 2
        },
        None
    )
    .is_ok());
}
#[test]
fn test_lease_window_overlap_rejected() {
    assert!(matches!(
        verify_lease_window(
            &LeaseWindow {
                start_tick: 2,
                end_tick: 4
            },
            Some(&LeaseWindow {
                start_tick: 1,
                end_tick: 2
            })
        ),
        Err(LeaseError::OverlappingWindow)
    ));
}
#[test]
fn test_lease_grant_valid() {
    let reg = LeaseRegistry {
        current_lease: lease(1, 1, 10),
    };
    let g = LeaseGrant {
        authority: reg.current_lease.authority,
        lease: lease(2, 11, 20),
    };
    assert!(verify_lease_grant(&g, &reg).is_ok());
}
#[test]
fn test_lease_expiration_detected() {
    assert!(matches!(
        verify_lease_expiration(&lease(1, 1, 2), 3),
        Err(LeaseError::LeaseExpired)
    ));
}
#[test]
fn test_execution_outside_window_rejected() {
    assert!(matches!(
        verify_lease_expiration(&lease(1, 5, 10), 4),
        Err(LeaseError::LeaseNotYetActive)
    ));
}
#[test]
fn test_lease_renewal_valid() {
    let prev = lease(1, 1, 10);
    let ren = LeaseRenewal {
        previous_lease_hash: hash_execution_lease(&prev),
        renewed_lease: lease(2, 11, 20),
    };
    assert!(verify_lease_renewal(&prev, &ren, false).is_ok());
}
#[test]
fn test_lease_renewal_continuity() {
    let prev = lease(1, 1, 10);
    let ren = LeaseRenewal {
        previous_lease_hash: hash_execution_lease(&prev),
        renewed_lease: lease(2, 12, 20),
    };
    assert!(matches!(
        verify_lease_renewal(&prev, &ren, false),
        Err(LeaseError::RenewalContinuityMismatch)
    ));
}
#[test]
fn test_lease_registry_monotonic() {
    let reg = LeaseRegistry {
        current_lease: lease(1, 1, 10),
    };
    assert!(update_lease_registry(&reg, &lease(2, 11, 20)).is_ok());
}
#[test]
fn test_overlapping_lease_rejected() {
    let reg = LeaseRegistry {
        current_lease: lease(1, 1, 10),
    };
    assert!(matches!(
        update_lease_registry(&reg, &lease(2, 10, 20)),
        Err(LeaseError::PolicyViolation)
    ));
}
#[test]
fn test_lease_policy_single_active() {
    assert!(matches!(
        verify_lease_policy(
            &LeasePolicy {
                single_active_lease: true
            },
            &lease(1, 1, 10),
            &lease(2, 9, 12)
        ),
        Err(LeaseError::PolicyViolation)
    ));
}
#[test]
fn test_lease_replay_consistency() {
    let reg = LeaseRegistry {
        current_lease: lease(1, 1, 10),
    };
    let a = verify_execution_lease(&reg.current_lease, &reg, 5).unwrap();
    let b = verify_execution_lease(&reg.current_lease, &reg, 5).unwrap();
    assert_eq!(a, b);
}
#[test]
fn test_lease_deterministic() {
    let l = lease(1, 1, 10);
    assert_eq!(l, lease(1, 1, 10));
}
