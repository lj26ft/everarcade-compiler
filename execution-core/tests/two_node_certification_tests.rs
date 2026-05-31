use execution_core::certification::two_node::{CertificationError, NodeRuntime};

fn joined_nodes() -> (NodeRuntime, NodeRuntime) {
    let node_a = NodeRuntime::boot_arena_vanguard("node-a");
    let node_b = NodeRuntime::join_from("node-b", &node_a).expect("node b joins from checkpoint");
    (node_a, node_b)
}

#[test]
fn test_node_join_certification() {
    let (node_a, node_b) = joined_nodes();
    let convergence = node_a
        .require_convergence(&node_b)
        .expect("joined nodes converge");
    assert!(convergence.converged());
    assert_eq!(node_a.world_root(), node_b.world_root());
    assert_eq!(node_a.replay_root(), node_b.replay_root());
    assert_eq!(
        node_a.checkpoint().checkpoint_root,
        node_b.checkpoint().checkpoint_root
    );
    assert_eq!(node_a.continuity_root(), node_b.continuity_root());
}

#[test]
fn test_world_convergence() {
    let (mut node_a, mut node_b) = joined_nodes();
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 1_000, 100)
        .expect("1000 deterministic ticks converge");
    assert_eq!(node_a.world_root(), node_b.world_root());
    assert!(node_a.require_convergence(&node_b).unwrap().converged());
}

#[test]
fn test_replay_convergence() {
    let (mut node_a, mut node_b) = joined_nodes();
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 64, 16).unwrap();
    assert_eq!(node_a.replay_root(), node_b.replay_root());
    assert_eq!(node_a.replay_hash(), node_b.replay_hash());
    assert_eq!(node_a.continuity_root(), node_b.continuity_root());
}

#[test]
fn test_checkpoint_convergence() {
    let (mut node_a, mut node_b) = joined_nodes();
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 128, 32).unwrap();
    let checkpoint_a = node_a.checkpoint();
    let checkpoint_b = node_b.checkpoint();
    assert_eq!(checkpoint_a.checkpoint_root, checkpoint_b.checkpoint_root);
    let (restored_a, recovery_a) =
        NodeRuntime::restore_from_checkpoint("node-a-restore", &node_a).unwrap();
    let (restored_b, recovery_b) =
        NodeRuntime::restore_from_checkpoint("node-b-restore", &node_b).unwrap();
    assert!(recovery_a.restored);
    assert!(recovery_b.restored);
    assert_eq!(restored_a.world_root(), restored_b.world_root());
}

#[test]
fn test_node_failure_survival() {
    let (mut node_a, mut node_b) = joined_nodes();
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 10, 5).unwrap();
    node_a.terminate();
    node_b.identity.authoritative = true;
    node_b
        .tick()
        .expect("surviving node continues authority after failover");
    assert!(node_b.session.active);
    assert!(!node_b.world_root().is_empty());
    assert!(!node_b.replay_root().is_empty());
}

#[test]
fn test_node_recovery_convergence() {
    let (mut node_a, mut node_b) = joined_nodes();
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 25, 5).unwrap();
    node_a.terminate();
    node_b.identity.authoritative = true;
    for _ in 0..10 {
        node_b.tick().unwrap();
    }
    let (recovered_a, recovery) = NodeRuntime::restore_from_checkpoint("node-a", &node_b).unwrap();
    assert!(recovery.restored);
    assert_eq!(
        recovery.source_checkpoint_root,
        recovery.restored_checkpoint_root
    );
    assert!(recovered_a
        .require_convergence(&node_b)
        .unwrap()
        .converged());
}

#[test]
fn test_divergence_detection() {
    let (node_a, mut node_b) = joined_nodes();
    node_b.identity.authoritative = true;
    node_b.mutate_authority_state(7).unwrap();
    let err = node_a.require_convergence(&node_b).unwrap_err();
    assert!(matches!(err, CertificationError::DivergenceDetected(_)));
}

#[test]
fn test_corrupt_checkpoint_rejection() {
    let (node_a, mut node_b) = joined_nodes();
    let log = node_b.reject_corrupt_checkpoint(node_a.checkpoint());
    assert!(log.detected);
    assert!(log.rejected);
    assert!(node_a.require_convergence(&node_b).unwrap().converged());
}

#[test]
fn test_corrupt_replay_rejection() {
    let (node_a, mut node_b) = joined_nodes();
    let log = node_b.reject_corrupt_replay();
    assert!(log.detected);
    assert!(log.rejected);
    assert!(node_a.require_convergence(&node_b).unwrap().converged());
}

#[test]
fn test_network_partition_detection() {
    let (mut node_a, mut node_b) = joined_nodes();
    node_a.partition();
    node_b.partition();
    node_a.tick().unwrap();
    node_b.independent_partition_activity().unwrap();
    node_a.reconnect();
    node_b.reconnect();
    assert!(node_a.detect_partition_divergence(&node_b).is_err());
    assert!(node_a.logs.iter().any(|log| log.detected && log.rejected));
}

#[test]
fn test_authority_preservation() {
    let (_node_a, mut node_b) = joined_nodes();
    assert!(matches!(
        node_b.mutate_authority_state(1),
        Err(CertificationError::AuthorityViolation(_))
    ));
    assert!(matches!(
        node_b.rewrite_replay("rewrite|forbidden"),
        Err(CertificationError::AuthorityViolation(_))
    ));
}

#[test]
fn test_arena_vanguard_two_node_runtime() {
    let (mut node_a, mut node_b) = joined_nodes();
    assert_eq!(node_a.session.world_id, "arena-vanguard");
    NodeRuntime::run_authoritative_ticks(&mut node_a, &mut node_b, 1_000, 100).unwrap();
    assert!(node_a.require_convergence(&node_b).unwrap().converged());
    let corrupt_checkpoint = node_b.reject_corrupt_checkpoint(node_a.checkpoint());
    let corrupt_replay = node_b.reject_corrupt_replay();
    assert!(corrupt_checkpoint.rejected);
    assert!(corrupt_replay.rejected);
}
