use everarcade_rustrig_governance::{
    finalize_proposal, replay_with_finalizations as replay, vote, GovernanceError, GovernanceState,
    Proposal, ProposalStatus, VoteInput, RUSTRIG_ID,
};

fn proposal(id: &str, quorum: u64) -> Proposal {
    Proposal::new(
        id,
        "Raise walls",
        "Fund city wall upgrades",
        "alice",
        1,
        10,
        20,
        quorum,
    )
}

fn fixture_state() -> GovernanceState {
    GovernanceState::new()
        .with_voter("alice")
        .with_voter("bob")
        .with_voter("carol")
        .with_proposal(proposal("p1", 2))
}

fn input(voter: &str, choice: &str, tick: u64) -> VoteInput {
    VoteInput {
        proposal_id: "p1".to_owned(),
        voter_id: voter.to_owned(),
        vote_choice: choice.to_owned(),
        tick,
    }
}

#[test]
fn valid_vote() {
    let output = vote(&fixture_state(), input("alice", "yes", 10)).unwrap();
    assert_eq!(output.receipt.proposal_id, "p1");
    assert_eq!(output.receipt.voter_id, "alice");
    assert_eq!(output.receipt.vote_choice, "yes");
    assert_eq!(output.receipt.tick, 10);
    assert_eq!(output.state.votes.len(), 1);
    assert_eq!(output.state.receipts.len(), 1);
    assert_eq!(RUSTRIG_ID, "governance.vote");
}

#[test]
fn proposal_missing() {
    let err = vote(
        &fixture_state(),
        VoteInput {
            proposal_id: "missing".to_owned(),
            ..input("alice", "yes", 10)
        },
    )
    .unwrap_err();
    assert_eq!(err, GovernanceError::ProposalMissing);
}

#[test]
fn voter_missing() {
    let err = vote(&fixture_state(), input("mallory", "yes", 10)).unwrap_err();
    assert_eq!(err, GovernanceError::VoterMissing);
}

#[test]
fn duplicate_vote() {
    let state = vote(&fixture_state(), input("alice", "yes", 10))
        .unwrap()
        .state;
    let err = vote(&state, input("alice", "no", 11)).unwrap_err();
    assert_eq!(err, GovernanceError::DuplicateVote);
}

#[test]
fn invalid_vote_choice() {
    let err = vote(&fixture_state(), input("alice", "maybe", 10)).unwrap_err();
    assert_eq!(err, GovernanceError::InvalidVoteChoice);
}

#[test]
fn vote_before_start_tick() {
    let err = vote(&fixture_state(), input("alice", "yes", 9)).unwrap_err();
    assert_eq!(err, GovernanceError::TickBeforeVotingWindow);
}

#[test]
fn vote_after_end_tick() {
    let err = vote(&fixture_state(), input("alice", "yes", 21)).unwrap_err();
    assert_eq!(err, GovernanceError::TickAfterVotingWindow);
}

#[test]
fn accepted_proposal() {
    let state = vote(&fixture_state(), input("alice", "yes", 10))
        .unwrap()
        .state;
    let state = vote(&state, input("bob", "yes", 11)).unwrap().state;
    let output = finalize_proposal(&state, "p1", 21).unwrap();
    assert_eq!(output.status, ProposalStatus::Accepted);
    assert_eq!(output.tally.yes_count, 2);
}

#[test]
fn rejected_proposal() {
    let state = vote(&fixture_state(), input("alice", "yes", 10))
        .unwrap()
        .state;
    let state = vote(&state, input("bob", "no", 11)).unwrap().state;
    let output = finalize_proposal(&state, "p1", 21).unwrap();
    assert_eq!(output.status, ProposalStatus::Rejected);
}

#[test]
fn quorum_failure() {
    let state = GovernanceState::new()
        .with_voter("alice")
        .with_proposal(proposal("p1", 2));
    let state = vote(&state, input("alice", "yes", 10)).unwrap().state;
    let output = finalize_proposal(&state, "p1", 21).unwrap();
    assert_eq!(output.status, ProposalStatus::Rejected);
    assert_eq!(output.tally.votes_cast(), 1);
}

#[test]
fn proposal_finality() {
    let state = vote(&fixture_state(), input("alice", "yes", 10))
        .unwrap()
        .state;
    let finalized = finalize_proposal(&state, "p1", 21).unwrap().state;
    assert_eq!(
        vote(&finalized, input("bob", "yes", 22)).unwrap_err(),
        GovernanceError::ProposalFinalized
    );
    assert_eq!(
        finalize_proposal(&finalized, "p1", 22).unwrap_err(),
        GovernanceError::ProposalFinalized
    );
}

#[test]
fn receipt_integrity() {
    let output = vote(&fixture_state(), input("bob", "abstain", 12)).unwrap();
    let receipt = output
        .state
        .receipts
        .get(&output.receipt.receipt_id)
        .unwrap();
    assert_eq!(receipt, &output.receipt);
    assert_eq!(receipt.proposal_id, "p1");
    assert_eq!(receipt.voter_id, "bob");
    assert_eq!(receipt.vote_choice, "abstain");
    assert_eq!(receipt.tick, 12);
}

#[test]
fn replay_equivalence() {
    let inputs = vec![
        input("alice", "yes", 10),
        input("bob", "no", 11),
        input("carol", "abstain", 12),
    ];
    let mut manual = fixture_state();
    for item in &inputs {
        manual = vote(&manual, item.clone()).unwrap().state;
    }
    manual = finalize_proposal(&manual, "p1", 21).unwrap().state;
    let replayed = replay(&fixture_state(), &inputs, &[("p1".to_owned(), 21)]).unwrap();
    assert_eq!(manual, replayed);
}

#[test]
fn root_equivalence() {
    let inputs = vec![input("alice", "yes", 10), input("bob", "yes", 11)];
    let replay_a = replay(&fixture_state(), &inputs, &[("p1".to_owned(), 21)]).unwrap();
    let replay_b = replay(&fixture_state(), &inputs, &[("p1".to_owned(), 21)]).unwrap();
    assert_eq!(replay_a.state_root(), replay_b.state_root());
}
