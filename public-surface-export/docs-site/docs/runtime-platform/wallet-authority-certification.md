# Wallet Authority Certification v0.1

## Purpose

Wallet Authority Certification proves that the EverArcade Runtime Platform can maintain deterministic permission state for asset movement requests across authority assignment, authorization, rejection, delegation, revocation, checkpointing, restoration, continued activity, and replay.

The certification validates runtime authority only. Authority determines who may authorize an action; it does not perform settlement, validate wallet cryptography, connect to XRPL, connect to Xaman, execute hooks, or implement multisig.

## Authority Model

Authority identities are deterministic runtime records such as `authority:a`, `authority:b`, and `delegate:c`.

They are not blockchain accounts, Xaman wallets, seed phrases, public keys, private keys, hooks, or multisig signers. Those systems remain outside this certification boundary.

Each certified asset is created with deterministic ownership, inventory membership, vault custody, and a single primary authority record. The authority lineage is part of the canonical transcript used to derive roots.

## Authorization Model

An authorization request is valid only when the requester is the asset's primary authority or an active delegate of that primary authority.

Authorized requests append deterministic authority lineage and advance the authority root. Unauthorized requests are rejected and must not mutate state. The script verifies this by comparing the authority root before and after rejected attempts.

## Delegation Model

Delegation records a deterministic relationship from a primary authority to a delegate for an asset movement permission path.

The certified flow delegates `authority:a` to `delegate:c` for `asset:sword-001`. Delegation must preserve owner, inventory, vault custody, primary authority, and lineage. Delegation does not transfer ownership or settle assets.

## Revocation Model

Revocation marks an active delegate as revoked while preserving the historical delegation lineage.

After revocation, the delegate can no longer authorize the movement request. A revoked delegate request must be rejected and must leave the authority state unchanged.

## Checkpoint Model

After deterministic authority evolution, the certification writes a checkpoint containing:

- Checkpoint version.
- Authority checkpoint identifier.
- Persisted authority root.
- Canonical authority transcript.

The checkpoint identifier is derived from the certification version, persisted authority root, and event log hash. Checkpoint verification requires the persisted identifier and root to match the canonical state.

## Replay Model

Replay independently executes the complete lifecycle:

```text
Genesis -> Assignment -> Authorization -> Delegation -> Delegated Action -> Revocation -> Rejection -> Evolution -> Checkpoint -> Restart -> Restore -> Continue Activity
```

Replay passes only when the replay authority root exactly matches the primary authority continuity root. A mismatch indicates nondeterministic ordering, checkpoint drift, restoration drift, lineage drift, invalid rejection handling, or hidden authority ambiguity.

## Authority Integrity Rules

The certification requires:

1. Every expected asset is created exactly once.
2. Every asset has exactly one runtime owner.
3. Every asset has owner-compatible inventory membership.
4. Every asset has deterministic vault custody.
5. Every certified asset has exactly one primary authority record.
6. Authority records reference known deterministic runtime identities.
7. Delegation references a known primary authority and known delegate.
8. Revocation preserves delegation lineage.
9. Unauthorized requests never approve.
10. Revoked delegates never approve.
11. Rejected requests do not mutate authority state.
12. Event identifiers are unique.
13. Authority roots evolve across Epoch 0, Epoch 1, and Epoch 2.
14. Replay reproduces the same authority continuity root.

## PASS Criteria

The certification passes when:

1. Runtime bootstrap certification is available and passing.
2. Asset setup creates deterministic assets, owners, inventory entries, and vault custody.
3. Authority assignment succeeds without ambiguity.
4. Authorized primary authority requests succeed.
5. Unauthorized authority requests are rejected without state mutation.
6. Delegation succeeds and preserves lineage.
7. Delegated authorization succeeds while delegation is active.
8. Revocation succeeds and preserves lineage.
9. Revoked authority requests are rejected without state mutation.
10. Authority roots evolve deterministically.
11. Checkpoint creation and verification pass.
12. Restoration recovers authority state, delegations, and revocations.
13. Continued activity advances authority continuity.
14. Replay produces an identical authority root.
15. Integrity validation confirms no duplicate authority, ambiguous authority, invalid delegation, unauthorized approval, revoked approval, replay divergence, or lineage corruption.

## FAIL Criteria

The certification fails if any required status is not `PASS`, including:

- Runtime bootstrap failure.
- Duplicate asset creation.
- Missing or ambiguous primary authority.
- Invalid delegation acceptance.
- Unauthorized request approval.
- Revoked authority approval.
- Rejected request state mutation.
- Lineage corruption.
- Non-evolving epoch roots.
- Checkpoint creation or verification failure.
- Restoration mismatch.
- Replay authority divergence.

## Relationship To Vault Ownership Certification

Vault Ownership Certification proves deterministic custody for owned assets. Wallet Authority Certification builds on that custody layer by proving who may authorize movement requests involving those assets.

Vaults answer where an asset is held. Authority answers who may authorize an action concerning that asset. Neither certification performs external settlement.

## Relationship To Future XRPL/Xaman Authority Certification

This certification intentionally excludes XRPL accounts, Xaman wallets, cryptographic signing, hooks, multisig, federation, and settlement.

Future XRPL/Xaman Authority Certification can map external wallet signatures and account relationships onto these deterministic runtime authority records while preserving the same checkpoint, restoration, replay, and continuity expectations.
