# Property Targets: governance.vote


## Property: One Vote Per Authority

Given:
Valid governance.vote pre-state satisfying declared state domains.

When:
`governance.vote()` executes under declared mutation authority.

Then:
A voting authority cannot cast duplicate effective votes for one proposal.


## Property: Eligibility Authorization

Given:
Valid governance.vote pre-state satisfying declared state domains.

When:
`governance.vote()` executes under declared mutation authority.

Then:
Only eligible authorities may vote.


## Property: Tally Continuity

Given:
Valid governance.vote pre-state satisfying declared state domains.

When:
`governance.vote()` executes under declared mutation authority.

Then:
Vote tallies remain replay-stable across deterministic execution.
