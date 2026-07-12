# Runtime Assembly Reference Resolution v1

`everarcade.symbol-table.v1`, `everarcade.reference-graph.v1`, and `everarcade.resolved-declarations.v1` define the Phase D boundary between deterministic contribution merge and Runtime IR construction.

## Pipeline

The compiler architecture is now:

```text
request → profile graph → contribution graph → deterministic merge → symbol/reference resolution → Runtime IR
```

Phase D does not populate the full Runtime IR. It registers authoritative merged declarations, resolves typed references, emits evidence, and hands a stable `ResolvedDeclarationSetV1` to the future Runtime IR phase.

## Namespaces

Symbols are namespace-scoped with canonical string names: `runtime`, `region`, `zone`, `passage`, `spawn_point`, `transition_point`, `interaction_point`, `entity`, `entity_archetype`, `item_archetype`, `encounter_archetype`, `item`, `primitive`, `action`, `transition`, `invariant`, `encounter`, `progression_rule`, `world_variable`, `capability`, `receipt_type`, and `proof_policy`.

A symbol in one namespace never satisfies a reference expecting another namespace.

## Symbol identity

A `SymbolIdentity` is the stable tuple of namespace, local id, schema version, declaration content hash, source contribution id, and source profile id. Its canonical display form is `namespace:local_id`. It does not depend on filesystem path, profile loading order, declaration order, merge traversal order, timestamps, or local machine state.

## Typed references

References carry an expected namespace and required/optional flag. V1 includes typed forms for regions, spawn points, entities, entity archetypes, item archetypes, primitives, actions, transitions, invariants, encounters, progression rules, world variables, capabilities, receipt types, and proof policies.

## Alias policy

Aliases are authoring-only, namespace-scoped metadata on symbol definitions. Canonical evidence records canonical ids, not aliases. Alias cycles are invalid and reported with `ASSEMBLY_REFERENCE_ALIAS_CYCLE` when encountered by future alias-expansion inputs.

## Generated IDs

Generated ids are deterministic and derive from a domain-separated tuple containing the world id, namespace, source profile, source contribution, declaration key, and schema version. UUIDs and timestamps are not used. Collisions are rejected with `ASSEMBLY_GENERATED_ID_COLLISION`.

## Reference graph and hashing

`ReferenceGraphV1` contains sorted symbol nodes and sorted reference edges. Edges record source and target namespaces, required/optional status, resolution status, selected targets, provenance, dependency depth, and compatibility metadata.

The graph hash uses the `everarcade.reference-graph.v1` domain and covers canonical symbol identities, declaration hashes, reference edges, expected namespaces, optional/required flags, resolved targets, and compatibility decisions. Loading order does not affect the hash.

## Required and optional references

Required missing references fail assembly diagnostics. Optional missing references produce explicit `optional_unresolved` graph edges and evidence entries when absence has deterministic neutral meaning.

## Semantic cycle policy

Topology loops and bidirectional region links are valid. Alias cycles, ownership cycles, transition dependency cycles that require acyclic execution, and progression prerequisite cycles are invalid. Cycle diagnostics are emitted by semantic graph domain rather than by rejecting every graph cycle.

## Reachability checks

Phase D performs structural reachability checks: initial entities must reference existing regions, transitions must have existing destinations and reachable declared actions, enabled actions must resolve handlers, and encounter spawn points must belong to declared regions or permitted linked regions. Full gameplay solvability is deferred.

## Version compatibility

References may include expected schema or major-version constraints. Primitive handler mismatches, action/transition schema mismatches, receipt-schema mismatches, and unsupported proof capabilities emit version or capability diagnostics and record compatibility decisions.

## Diagnostics

Structured diagnostics include duplicate symbols, namespace conflicts, invalid ids, generated-id collisions, missing/ambiguous references, namespace mismatches, version mismatches, disabled targets, alias/ownership/dependency cycles, unreachable references, and unresolved optional references. Diagnostics include source declaration, expected namespace, referenced id, candidate targets where relevant, provenance, and remediation guidance.
