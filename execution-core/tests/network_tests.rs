use execution_core::network::*;

fn sample_nodes() -> Vec<NetworkNode> {
    vec![
        NetworkNode {
            node_id: "exec-a".into(),
            supported_epochs: vec![1, 2],
            execution_capabilities: vec!["wasm".into()],
            verifier: false,
            archive: false,
            execution: true,
            reputation_score: 10,
        },
        NetworkNode {
            node_id: "exec-b".into(),
            supported_epochs: vec![1, 2],
            execution_capabilities: vec!["wasm".into()],
            verifier: false,
            archive: false,
            execution: true,
            reputation_score: 8,
        },
        NetworkNode {
            node_id: "verifier-1".into(),
            supported_epochs: vec![1, 2],
            execution_capabilities: vec![],
            verifier: true,
            archive: false,
            execution: false,
            reputation_score: 7,
        },
    ]
}

#[test]
fn test_cross_node_execution() {
    let pkg = b"deterministic-package";
    let p1 = TransportService::package("pkg-1", pkg, 4);
    let p2 = TransportService::package("pkg-1", pkg, 7);
    let (a, _) = TransportService::reconstruct(&p1).unwrap();
    let (b, _) = TransportService::reconstruct(&p2).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_execution_routing_stability() {
    let nodes = sample_nodes();
    let table = RoutingTable::default();
    let d1 = table.select_execution_node("pkg", 1, &nodes).unwrap();

    let mut reversed = nodes.clone();
    reversed.reverse();
    let d2 = table.select_execution_node("pkg", 1, &reversed).unwrap();
    assert_eq!(d1.selected_node_id, d2.selected_node_id);
}

#[test]
fn test_distributed_replay_consensus() {
    let votes = vec![
        ConsensusVote {
            verifier_id: "v1".into(),
            receipt_hash: "abc".into(),
        },
        ConsensusVote {
            verifier_id: "v2".into(),
            receipt_hash: "abc".into(),
        },
        ConsensusVote {
            verifier_id: "v3".into(),
            receipt_hash: "abc".into(),
        },
    ];
    assert!(ReplayConsensus::agrees(&votes, 2));
}

#[test]
fn test_execution_claim_validation() {
    let valid = ExecutionClaim {
        package_id: "pkg".into(),
        node_id: "exec-a".into(),
        receipt_hash: "r1".into(),
        claimed_epoch: 2,
    };
    let invalid = ExecutionClaim {
        package_id: "pkg".into(),
        node_id: "exec-a".into(),
        receipt_hash: "r2".into(),
        claimed_epoch: 1,
    };

    assert!(ExecutionClaimValidator::validate(&valid, 2, "r1"));
    assert!(!ExecutionClaimValidator::validate(&invalid, 2, "r1"));
}

#[test]
fn test_package_transport_integrity() {
    let bytes = b"portable deterministic execution package";
    let packet = TransportService::package("pkg", bytes, 5);
    let (rebuilt, receipt) = TransportService::reconstruct(&packet).unwrap();
    assert_eq!(rebuilt, bytes);
    assert_eq!(receipt.chunk_count, packet.chunks.len());
}

#[test]
fn test_epoch_aware_network_execution() {
    let discovery = NodeDiscovery::new(sample_nodes());
    let epoch_nodes = discovery.discover_for_epoch(2);
    assert!(epoch_nodes.iter().all(|n| n.supports_epoch(2)));
    assert_eq!(epoch_nodes.len(), 3);
}

#[test]
fn test_execution_market_determinism() {
    let demand = ExecutionDemand {
        package_id: "pkg".into(),
        required_capacity: 50,
    };
    let supplies = vec![
        ExecutionSupply {
            node_id: "n2".into(),
            offered_capacity: 90,
        },
        ExecutionSupply {
            node_id: "n1".into(),
            offered_capacity: 50,
        },
    ];
    let chosen = ExecutionMarket::match_supply(&demand, &supplies).unwrap();
    assert_eq!(chosen.node_id, "n1");
}
