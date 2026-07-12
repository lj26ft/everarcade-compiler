# Runtime Assembly Runtime IR v1

`PtwRuntimeIrV1` is the authoritative, deterministic world declaration produced by the compiler after request normalization, profile resolution, contribution graph construction, deterministic merge, and reference resolution. Package emission is a lowering step from this IR; emitters must serialize and normalize IR content and must not invent missing authoritative state.

## Architecture

```text
request
→ profile graph
→ contribution graph
→ deterministic merge
→ reference resolution
→ Runtime IR construction
→ validation
→ lowering
```

## Authoritative field model

Runtime IR v1 records runtime metadata, graph identities, compiler/runtime capability requirements, explicit limits, numeric/coordinate/tick models, deterministic ID policy, topology, spawn points, archetypes, initial entities, world variables, player state template, primitives, actions, transition bindings, invariants, encounters, progression, authoritative content, checkpoint/journal/root policies, state domains, and proof-readiness metadata. Projection-only declarations stay outside authoritative runtime fields.

## Universal defaults

The only universal defaults are versioned and serialized in `universal_defaults`:

- `initial_tick = 0`.
- Optional collections are empty when absent.
- Optional primitives are represented explicitly as disabled.

Topology, spawn positions, limits, enabled actions, primitive handlers, entity archetypes, and proof policies are never hidden defaults.

## Limits

Runtime IR requires explicit bounded limits for players, entities, encounters, inventory, item stacks, health, damage, movement delta, spawning, actions, payload size, world variables, topology nodes, transition preconditions, invariant checks, and progression tiers. Values must be non-zero, deterministic, and within compiler policy. Effective limit provenance is preserved in `limit_provenance`.

## Numeric model

The authoritative numeric model is bounded integer based with declared bit width and explicit overflow, saturation, and rounding behavior. Authoritative floating-point numeric models are rejected because deterministic encoding and replay semantics are not part of Runtime IR v1.

## State domains

Runtime IR declares domains for future component roots and bounded witnesses: `players`, `entities`, `inventory`, `topology_state`, `encounters`, `progression`, `world_variables`, `economy`, and `governance`.

## Topology and entities

Regions, zones, passages, spawn points, transition points, interaction points, archetypes, and initial entities are canonical IR fields. Construction validates required topology presence and reference-resolution diagnostics from the resolved declaration set.

## Primitives, actions, transitions, and invariants

Primitive configurations are explicit for identity, movement, health, combat, inventory, items, spawning, encounters, progression, and interactions. Enabled primitives must have handlers. Actions must declare bounded payloads and an enabled primitive dependency. Invariants are copied from authoritative contributions and participate in validation and proof-readiness counts.

## Encounters and progression

Encounter and progression declarations support disabled/empty forms. When populated, they are bounded by required encounter and progression limits and reference resolution diagnostics.

## Proof readiness

`proof_readiness` records bounded transition status, numeric compatibility, maximum touched entities per action, maximum invariant checks, receipt coverage, checkpoint support, journal support, root support, and future ZK witness compatibility status. This phase does not generate ZK proofs.

## Runtime IR hash

Runtime IR v1 uses the domain separator `everarcade.runtime-ir.v1`. The hash covers canonical authoritative IR and changes when runtime-authoritative fields change. Projection-only metadata, local file paths, and diagnostic formatting are excluded from the production hash boundary.

## Lowering boundary and minimal-runtime removal

Production world creation calls the canonical Runtime IR builder and lowers from `PtwRuntimeIrV1` into package evidence, including `metadata/runtime-ir.json`, `metadata/runtime-ir-validation.json`, and `proof/assembly-manifest.json`. The minimal runtime fixture is not a production fallback; incomplete production requests fail with structured diagnostics.
