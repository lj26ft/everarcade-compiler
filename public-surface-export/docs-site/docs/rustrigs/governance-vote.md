# governance.vote()

`governance.vote()` is the certified EverArcade RustRig for deterministic institutional behavior. It provides the canonical reference implementation for casting governance votes, validating quorum, finalizing proposals, generating deterministic receipts, and replaying governance history.

World Contracts declare the primitive as a certified mutation:

```toml
[rustrigs.governance.vote]
crate = "everarcade-rustrig-governance"
certification = "PASS"
authority = "world"
receipt = "VoteReceipt"
```

## Proposal lifecycle

A proposal contains a `proposal_id`, title, description, proposer, creation tick, voting window, status, and integer quorum. Proposal status is one of `active`, `accepted`, `rejected`, or `expired`. The initial certified implementation casts votes only against `active` proposals and finalizes active proposals after their voting window closes.

## Voting windows

Voting is defined only by deterministic ticks. There are no timestamps, wall-clock reads, randomness, or off-chain voting inputs. A vote is accepted only when `tick >= voting_start_tick` and `tick <= voting_end_tick`.

## Quorum rules

The v1 quorum model is integer-only:

```text
votes_cast >= quorum
```

No percentages, floating point arithmetic, or fractional thresholds are used.

## Vote casting

A vote input contains `proposal_id`, `voter_id`, `vote_choice`, and `tick`. Valid choices are `yes`, `no`, and `abstain`. A vote is valid only when the proposal exists, the voter exists, the proposal is active and not finalized, the tick is inside the voting window, the vote choice is valid, and the voter has not already voted on that proposal.

## Proposal finalization

`governance.finalize_proposal()` may run only when `tick > voting_end_tick`. It computes exact tallies for yes, no, and abstain votes. A proposal is accepted only when `yes_count > no_count` and quorum is satisfied. Otherwise, it is rejected. Finalized proposals may not be voted on or finalized again.

## Receipt generation

`VoteReceipt` records `receipt_id`, `proposal_id`, `voter_id`, `vote_choice`, and `tick`. The receipt id is derived deterministically from the RustRig id, version, and vote input, so equivalent replay produces identical receipts.

## Replay behavior

Replay applies the same ordered vote inputs and finalization actions to the same initial state. Because state maps are canonicalized in deterministic order and receipt ids are deterministic, replay produces identical tallies, outcomes, receipts, and state roots.

## Certification requirements

The certified implementation passes:

- RR-GOV-001 Vote Determinism
- RR-GOV-002 Vote Uniqueness
- RR-GOV-003 Tally Integrity
- RR-GOV-004 Quorum Integrity
- RR-GOV-005 Proposal Finality
- RR-GOV-006 Receipt Integrity
- RR-GOV-007 Replay Equivalence
- RR-GOV-008 Root Equivalence
- RR-GOV-009 Tick Window Integrity

Certification output:

```text
RUSTRIG GOVERNANCE VOTE CERTIFICATION: PASS
```

## Future governance RustRigs

Future governance RustRigs must inherit the certification model established by `governance.vote()`:

- `governance.create_proposal()`
- `governance.cancel_proposal()`
- `governance.delegate_vote()`
- `governance.amend_constitution()`
- `governance.create_faction()`
- `governance.create_guild()`
- `governance.elect_leader()`
