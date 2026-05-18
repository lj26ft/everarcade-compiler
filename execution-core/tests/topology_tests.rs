use std::collections::{BTreeMap, BTreeSet};

use execution_core::{
    federation::node::FederationNodeId,
    sync::{advertisement::ContinuityAdvertisement, cursor::SyncCursor},
    topology::{
        convergence::verify_convergence,
        fanout::deterministic_fanout,
        gossip::{verify_gossip, GossipAdvertisement},
        health::{verify_observer_health, ObserverHealth},
        mesh::{mesh_hash, ObserverMesh, ObserverNode},
        neighbors::deterministic_neighbors,
        propagation::{advance_propagation, PropagationState},
        routing::deterministic_route,
        subscriptions::{verify_subscription, Subscription},
    },
};

fn n(v: u8) -> FederationNodeId {
    FederationNodeId::new([v; 32])
}
fn cursor(seq: u64) -> SyncCursor {
    SyncCursor {
        latest_sequence: seq,
        latest_execution_id: [1; 32],
        latest_checkpoint_root: [2; 32],
        latest_manifest_hash: [3; 32],
        latest_lineage_hash: [4; 32],
    }
}
fn mesh() -> ObserverMesh {
    let mut observers = BTreeMap::new();
    for i in 1..=3 {
        observers.insert(
            n(i),
            ObserverNode {
                node_id: n(i),
                latest_cursor: cursor(10),
                synchronized: true,
            },
        );
    }
    ObserverMesh { observers }
}

#[test]
fn test_mesh_hash_stable() {
    let m = mesh();
    assert_eq!(mesh_hash(&m), mesh_hash(&m));
}
#[test]
fn test_neighbor_selection_deterministic() {
    let m = mesh();
    assert_eq!(
        deterministic_neighbors(&m, n(1), 2),
        deterministic_neighbors(&m, n(1), 2)
    );
}
#[test]
fn test_gossip_validation() {
    let m = mesh();
    let g = GossipAdvertisement {
        source: n(1),
        advertisement: ContinuityAdvertisement {
            world_id: "w".into(),
            operator: n(1),
            cursor: cursor(10),
            package_root: [9; 32],
            checkpoint_root: [2; 32],
        },
    };
    assert!(verify_gossip(&m, &g, &BTreeSet::new()).is_ok());
}
#[test]
fn test_duplicate_gossip_rejected() {
    let m = mesh();
    let g = GossipAdvertisement {
        source: n(1),
        advertisement: ContinuityAdvertisement {
            world_id: "w".into(),
            operator: n(1),
            cursor: cursor(10),
            package_root: [9; 32],
            checkpoint_root: [2; 32],
        },
    };
    let mut seen = BTreeSet::new();
    seen.insert((n(1), 10));
    assert!(verify_gossip(&m, &g, &seen).is_err());
}
#[test]
fn test_propagation_advancement_monotonic() {
    let mut s = PropagationState {
        propagated_to: BTreeSet::new(),
        propagation_complete: false,
    };
    assert!(advance_propagation(&mut s, n(1), 2));
    assert!(!advance_propagation(&mut s, n(1), 2));
    assert!(advance_propagation(&mut s, n(2), 2));
    assert!(s.propagation_complete);
}
#[test]
fn test_fanout_deterministic() {
    let m = mesh();
    assert_eq!(
        deterministic_fanout(&m, n(1), 2),
        deterministic_fanout(&m, n(1), 2)
    );
}
#[test]
fn test_route_deterministic() {
    let p = vec![n(1), n(2), n(3)];
    assert_eq!(
        deterministic_route(p.clone()).unwrap(),
        deterministic_route(p).unwrap()
    );
}
#[test]
fn test_route_cycle_rejected() {
    assert!(deterministic_route(vec![n(1), n(2), n(1)]).is_err());
}
#[test]
fn test_subscription_validation() {
    let s = Subscription {
        observer: n(1),
        world_id: "w".into(),
    };
    assert!(verify_subscription(&s, &BTreeSet::from(["w".to_string()])).is_ok());
}
#[test]
fn test_convergence_success() {
    let r = verify_convergence(&mesh());
    assert!(r.converged);
}
#[test]
fn test_convergence_divergence_detected() {
    let mut m = mesh();
    m.observers.get_mut(&n(3)).unwrap().latest_cursor = cursor(9);
    let r = verify_convergence(&m);
    assert!(!r.converged);
    assert_eq!(r.diverged_observers, 1);
}
#[test]
fn test_observer_health_validation() {
    assert!(verify_observer_health(&ObserverHealth {
        node_id: n(1),
        synchronized: false,
        latest_sequence: 1
    }));
}
#[test]
fn test_topology_propagation_order_stable() {
    let m = mesh();
    let a = deterministic_fanout(&m, n(1), 10).targets;
    let b = deterministic_fanout(&m, n(1), 10).targets;
    assert_eq!(a, b);
}
#[test]
fn test_topology_replay_consistency() {
    let m = mesh();
    assert_eq!(mesh_hash(&m), mesh_hash(&m));
}
#[test]
fn test_mesh_convergence_deterministic() {
    let m = mesh();
    assert_eq!(verify_convergence(&m), verify_convergence(&m));
}
