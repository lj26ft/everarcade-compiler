# Property Targets: resource.harvest


## Property: Declared Mutation Safety

Given:
Valid resource.harvest pre-state satisfying declared state domains.

When:
`resource.harvest()` executes under declared mutation authority.

Then:
resource.harvest must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid resource.harvest pre-state satisfying declared state domains.

When:
`resource.harvest()` executes under declared mutation authority.

Then:
resource.harvest must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid resource.harvest pre-state satisfying declared state domains.

When:
`resource.harvest()` executes under declared mutation authority.

Then:
resource.harvest must emit replay-stable evidence for verifier review.

# Property Targets: resource.regenerate


## Property: Declared Mutation Safety

Given:
Valid resource.regenerate pre-state satisfying declared state domains.

When:
`resource.regenerate()` executes under declared mutation authority.

Then:
resource.regenerate must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid resource.regenerate pre-state satisfying declared state domains.

When:
`resource.regenerate()` executes under declared mutation authority.

Then:
resource.regenerate must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid resource.regenerate pre-state satisfying declared state domains.

When:
`resource.regenerate()` executes under declared mutation authority.

Then:
resource.regenerate must emit replay-stable evidence for verifier review.
