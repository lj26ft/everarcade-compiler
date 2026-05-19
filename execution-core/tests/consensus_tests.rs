use std::collections::BTreeMap;

use execution_core::consensus::*;
use execution_core::federation::node::FederationNodeId;

fn node(v: u8) -> FederationNodeId {
    FederationNodeId([v; 32])
}

#[test]
fn test_consensus_descriptor_hash_stable() {
    let d = ConsensusDescriptor {
        consensus_id: [1; 32],
        epoch_hash: [2; 32],
        quorum_hash: [3; 32],
    };
    assert_eq!(hash_consensus_descriptor(&d), hash_consensus_descriptor(&d));
}
#[test]
fn test_consensus_epoch_valid() {
    let p = ConsensusEpoch {
        epoch_number: 0,
        previous_epoch_hash: [0; 32],
    };
    let n = ConsensusEpoch {
        epoch_number: 1,
        previous_epoch_hash: hash_consensus_epoch(&p),
    };
    assert!(verify_consensus_epoch(&p, &n).is_ok());
}
#[test]
fn test_consensus_epoch_rollback_rejected() {
    let p = ConsensusEpoch {
        epoch_number: 1,
        previous_epoch_hash: [0; 32],
    };
    let n = ConsensusEpoch {
        epoch_number: 1,
        previous_epoch_hash: [0; 32],
    };
    assert!(verify_consensus_epoch(&p, &n).is_err());
}
#[test]
fn test_consensus_proposal_valid() {
    let mut p = ConsensusProposal {
        proposal_id: [0; 32],
        checkpoint_root: [9; 32],
        proposed_by: node(1),
    };
    p.proposal_id = hash_consensus_proposal(&p);
    assert!(verify_consensus_proposal(&p, [9; 32]).is_ok());
}
#[test]
fn test_duplicate_proposal_rejected() {
    let epoch = ConsensusEpoch {
        epoch_number: 0,
        previous_epoch_hash: [0; 32],
    };
    let mut map = BTreeMap::new();
    let mut p = ConsensusProposal {
        proposal_id: [0; 32],
        checkpoint_root: [1; 32],
        proposed_by: node(1),
    };
    p.proposal_id = hash_consensus_proposal(&p);
    map.insert(p.proposal_id, p.clone());
    let reg = ConsensusRegistry {
        active_epoch: epoch.clone(),
        active_proposals: map,
    };
    let next = ConsensusEpoch {
        epoch_number: 1,
        previous_epoch_hash: hash_consensus_epoch(&epoch),
    };
    assert!(update_consensus_registry(&reg, next, p).is_err());
}
#[test]
fn test_consensus_quorum_valid() {
    let q = ConsensusQuorum { required_nodes: 2 };
    assert!(verify_consensus_quorum(&q, &[node(1), node(2)]).is_ok());
}
#[test]
fn test_consensus_registry_stable() {
    let epoch = ConsensusEpoch {
        epoch_number: 0,
        previous_epoch_hash: [0; 32],
    };
    let reg = ConsensusRegistry {
        active_epoch: epoch,
        active_proposals: BTreeMap::new(),
    };
    assert_eq!(hash_consensus_registry(&reg), hash_consensus_registry(&reg));
}
#[test]
fn test_consensus_verification_valid() {
    let epoch = ConsensusEpoch {
        epoch_number: 0,
        previous_epoch_hash: [0; 32],
    };
    let mut p = ConsensusProposal {
        proposal_id: [0; 32],
        checkpoint_root: [1; 32],
        proposed_by: node(1),
    };
    p.proposal_id = hash_consensus_proposal(&p);
    let mut map = BTreeMap::new();
    map.insert(p.proposal_id, p.clone());
    let reg = ConsensusRegistry {
        active_epoch: epoch.clone(),
        active_proposals: map,
    };
    let state = ConsensusState {
        current_epoch: epoch,
        registry_hash: hash_consensus_registry(&reg),
    };
    let r = verify_consensus(
        &reg,
        &p,
        [1; 32],
        &ConsensusQuorum { required_nodes: 1 },
        &[node(1)],
        &ConsensusPolicy {
            proposals_required: true,
        },
        &state,
    )
    .unwrap();
    assert!(r.valid && r.quorum_valid);
}
#[test]
fn test_consensus_policy_requires_proposals() {
    let reg = ConsensusRegistry {
        active_epoch: ConsensusEpoch {
            epoch_number: 0,
            previous_epoch_hash: [0; 32],
        },
        active_proposals: BTreeMap::new(),
    };
    assert!(verify_consensus_policy(
        &ConsensusPolicy {
            proposals_required: true
        },
        &reg
    )
    .is_err());
}
#[test]
fn test_consensus_state_valid() {
    let reg = ConsensusRegistry {
        active_epoch: ConsensusEpoch {
            epoch_number: 0,
            previous_epoch_hash: [0; 32],
        },
        active_proposals: BTreeMap::new(),
    };
    let s = ConsensusState {
        current_epoch: reg.active_epoch.clone(),
        registry_hash: hash_consensus_registry(&reg),
    };
    assert!(verify_consensus_state(&s, &reg).is_ok());
}
#[test]
fn test_consensus_replay_consistency() {
    let reg = ConsensusRegistry {
        active_epoch: ConsensusEpoch {
            epoch_number: 0,
            previous_epoch_hash: [0; 32],
        },
        active_proposals: BTreeMap::new(),
    };
    let s = ConsensusState {
        current_epoch: reg.active_epoch.clone(),
        registry_hash: hash_consensus_registry(&reg),
    };
    assert_eq!(hash_consensus_state(&s), hash_consensus_state(&s));
}
#[test]
fn test_consensus_deterministic() {
    let q = ConsensusQuorum { required_nodes: 2 };
    assert!(verify_consensus_quorum(&q, &[node(2), node(1)]).is_ok());
}
