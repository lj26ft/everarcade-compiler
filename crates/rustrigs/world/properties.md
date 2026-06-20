# Property Targets: world.spawn


## Property: Declared Mutation Safety

Given:
Valid world.spawn pre-state satisfying declared state domains.

When:
`world.spawn()` executes under declared mutation authority.

Then:
world.spawn must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid world.spawn pre-state satisfying declared state domains.

When:
`world.spawn()` executes under declared mutation authority.

Then:
world.spawn must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid world.spawn pre-state satisfying declared state domains.

When:
`world.spawn()` executes under declared mutation authority.

Then:
world.spawn must emit replay-stable evidence for verifier review.

# Property Targets: world.despawn


## Property: Declared Mutation Safety

Given:
Valid world.despawn pre-state satisfying declared state domains.

When:
`world.despawn()` executes under declared mutation authority.

Then:
world.despawn must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid world.despawn pre-state satisfying declared state domains.

When:
`world.despawn()` executes under declared mutation authority.

Then:
world.despawn must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid world.despawn pre-state satisfying declared state domains.

When:
`world.despawn()` executes under declared mutation authority.

Then:
world.despawn must emit replay-stable evidence for verifier review.
