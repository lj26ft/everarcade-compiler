# Multi-Package Isolation Certification

## Purpose

The Multi-Package Isolation Certification proves that the runtime platform can host more than one persistent deterministic package at the same time without mixing package identity, state roots, checkpoints, continuity roots, or replay results.

It extends the single-package persistence certification by validating two package lifecycles in one certification surface:

- Package A executes, persists, restores, continues, and replays.
- Package B executes, persists, restores, continues, and replays.
- Neither lifecycle mutates, restores from, or validates against the other lifecycle.

## Isolation Guarantees

A PASS result means:

- Package identifiers are distinct.
- Initial state roots are package-scoped.
- Continued state roots are package-scoped.
- Checkpoints are bound to the package identifier that created them.
- Package A operations do not alter Package B persisted state.
- Package B operations do not alter Package A persisted state.
- Checkpoint A is rejected for Package B.
- Checkpoint B is rejected for Package A.

## Persistence Guarantees

A PASS result means each package writes an independent checkpoint record containing:

- Checkpoint version.
- Package identifier.
- Checkpoint identifier.
- Persisted state root.
- Persisted execution root.

Checkpoint identifiers must differ because they include package identity and package-specific state material.

## Restoration Guarantees

A PASS result means each package restores only from its own checkpoint. Restoration requires the checkpoint package identifier, checkpoint identifier, and persisted state root to match the expected package lifecycle.

A checkpoint created for one package must not satisfy restoration for the other package.

## Replay Guarantees

A PASS result means deterministic replay of each package lifecycle regenerates the same continuity root produced by the original lifecycle:

- Replay Root A equals Continuity Root A.
- Replay Root B equals Continuity Root B.
- Replay roots remain distinct across packages.

## PASS Criteria

The certification passes only when all of the following are true:

1. Runtime bootstrap certification passes.
2. Package Identifier A and Package Identifier B differ.
3. State Root A1 and State Root B1 differ.
4. Checkpoint A and Checkpoint B differ.
5. Package A restores from Checkpoint A.
6. Package B restores from Checkpoint B.
7. Cross-package checkpoint restoration is rejected.
8. Package-specific operations do not mutate the other package's persisted state.
9. Continuity Root A and Continuity Root B remain independent.
10. Replay Root A equals Continuity Root A.
11. Replay Root B equals Continuity Root B.

## FAIL Criteria

The certification fails if any of the following occur:

- Runtime bootstrap certification fails.
- Package identifiers collide.
- State roots or checkpoints collide across packages.
- A package restores from another package's checkpoint.
- Operations for one package change the other package's persisted state.
- Continuity roots converge unexpectedly across packages.
- Replay cannot reproduce either package continuity root.

## Relationship To Stateful Package Persistence

Stateful Package Persistence proves one deterministic package can execute, persist, restore, continue, and replay after restart.

Multi-Package Isolation uses the same persistence and replay model but runs two independent package identities through the lifecycle simultaneously. It certifies tenancy behavior: multiple long-lived deterministic packages can coexist without state leakage, cross-package checkpoint corruption, or continuity divergence.
