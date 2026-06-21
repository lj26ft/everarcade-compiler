# Tenant Runtime Certification v0.1

## Purpose

Tenant Runtime Certification proves that the EverArcade Runtime Platform can host multiple independent tenants in the same deterministic runtime surface without mixing package ownership, state roots, checkpoints, journals, continuity chains, or replay results.

It extends package-level certification by validating two tenant lifecycles at once:

- Tenant A owns Package A1 and Package A2.
- Tenant B owns Package B1 and Package B2.
- Each tenant executes, persists, restores, journals, continues, and replays independently.
- Neither tenant can satisfy validation with the other tenant's runtime material.

## Tenant Guarantees

A PASS result means:

- Tenant identifiers are deterministic and distinct.
- Tenant-scoped state roots include tenant identity and package membership.
- Tenant A state material differs from Tenant B state material.
- Tenant continuity roots remain independent after restore and continuation.
- Tenant replay roots remain independent and reproduce the original continuity roots.

## Ownership Guarantees

A PASS result means package allocations are tenant-owned:

- Package A1 and Package A2 are allocated only to Tenant A.
- Package B1 and Package B2 are allocated only to Tenant B.
- Allocation records bind package identifiers to the owning tenant identifier.
- Cross-tenant package membership is rejected by the certification surface.

## Persistence Guarantees

A PASS result means each tenant writes an independent checkpoint and persistence record containing:

- Tenant name.
- Tenant identifier.
- Tenant checkpoint identifier.
- Persisted tenant state root.
- Persisted tenant execution root.
- Tenant-owned package identifiers.

Checkpoint identifiers must differ because they include tenant identity, package membership, and tenant-specific state material.

## Journal Guarantees

A PASS result means each tenant writes an independent journal record bound to its tenant identifier.

Tenant A journal access requires Tenant A's identifier. Tenant B journal access requires Tenant B's identifier. The certification rejects Tenant A reading Tenant B's journal and Tenant B reading Tenant A's journal, and verifies journal entries do not contain cross-tenant package ownership.

## Replay Guarantees

A PASS result means deterministic replay of each tenant lifecycle regenerates the same continuity root produced by the original lifecycle:

- Replay Root A equals Continuity Root A.
- Replay Root B equals Continuity Root B.
- Replay roots remain distinct across tenants.

## PASS Criteria

The certification passes only when all of the following are true:

1. Runtime bootstrap certification passes.
2. Tenant Identifier A and Tenant Identifier B differ.
3. Tenant package allocations match the owning tenant.
4. Tenant A State Root and Tenant B State Root differ.
5. Tenant A Checkpoint and Tenant B Checkpoint differ.
6. Tenant A restores only Tenant A state.
7. Tenant B restores only Tenant B state.
8. Cross-tenant checkpoint restoration is rejected.
9. Tenant journals are inaccessible to the other tenant.
10. Continuity Root A and Continuity Root B remain independent.
11. Replay Root A equals Continuity Root A.
12. Replay Root B equals Continuity Root B.

## FAIL Criteria

The certification fails if any of the following occur:

- Runtime bootstrap certification fails.
- Tenant identifiers collide.
- Package ownership crosses tenant boundaries.
- State roots, checkpoints, journals, or continuity roots collide across tenants.
- A tenant restores from another tenant's checkpoint.
- A tenant accesses another tenant's journal.
- Replay cannot reproduce either tenant continuity root.

## Relationship To Multi-Package Isolation

Multi-Package Isolation proves that multiple deterministic package identities can execute, persist, restore, continue, and replay without crossing package boundaries.

Tenant Runtime Certification uses the same deterministic persistence and replay model but raises the boundary from packages to tenants. It certifies that groups of packages owned by independent operators remain isolated across state, checkpoints, journals, continuity, and replay, making tenant-level runtime hosting safe without adding networking, federation, XRPL, economic, or vault features.
