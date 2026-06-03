# Asset Ownership Continuity Certification v0.1

## Purpose

Asset Ownership Continuity Certification proves that the EverArcade Runtime Platform can maintain deterministic runtime possession for uniquely identified assets across genesis, assignment, transfer, checkpointing, restoration, continued activity, and replay.

This certification validates runtime asset ownership only. It does not validate wallet authority, XRPL ownership, Xaman signatures, vault custody, settlement, marketplace execution, federation settlement, multisig, or external identity systems.

## Asset Model

The certification creates three deterministic, uniquely identified assets:

- `asset:sword-001` (`Sword #001`).
- `asset:shield-001` (`Shield #001`).
- `asset:land-parcel-001` (`Land Parcel #001`).

Asset genesis passes only when every expected asset exists once and no duplicate asset identifiers are accepted. Asset roots are SHA-256 hashes over the canonical certification version, authority boundary, fixed asset order, fixed owner order, asset owners, lineage, and event log.

## Ownership Model

Ownership is modeled as exactly one owner slot per unique asset. The certification uses deterministic runtime owners:

- `player-a`.
- `player-b`.
- `world-treasury`.

An asset is valid only after assignment gives it one non-empty owner. Runtime ownership explicitly excludes wallet authority, XRPL authority, Xaman signature authority, vault custody authority, and network ordering authority.

## Transfer Model

A transfer is a deterministic ownership event with:

- Event identifier.
- Asset identifier.
- Expected source owner.
- Destination owner.

A transfer passes only when the current owner matches the expected source owner and the destination owner is non-empty and different from the source owner. Rejected transfers do not mutate ownership state.

## Ownership Lineage

Each asset records canonical lineage beginning at `genesis:none`, followed by assignment and transfer events. Lineage is valid only when the last lineage owner matches the current owner.

The certification rejects duplicate event identifiers, duplicate ownership, missing owners, orphaned assets, ambiguous lineage, and ownership mismatches.

## Checkpoint Model

After deterministic ownership evolution, the certification writes an ownership checkpoint containing:

- Checkpoint version.
- Ownership checkpoint identifier.
- Persisted ownership root.
- Canonical ownership transcript.

The checkpoint identifier is derived from the certification version, persisted ownership root, and event log hash. Verification requires the persisted identifier and ownership root to match the canonical state.

## Replay Model

Replay independently executes the complete lifecycle:

```text
Genesis -> Assignment -> Transfers -> Ownership Evolution -> Checkpoint -> Restart -> Restore -> Continue Ownership Activity
```

Replay passes only when the replay ownership root exactly matches the primary ownership continuity root. A mismatch indicates nondeterministic event order, checkpoint drift, restoration drift, lineage drift, or hidden ownership ambiguity.

## Integrity Rules

The certification requires:

1. Every expected asset is created exactly once.
2. Every asset has exactly one owner after assignment.
3. No asset is orphaned after assignment.
4. Transfers require the expected current owner.
5. Rejected transfers do not mutate state.
6. Lineage exists for every asset.
7. The current owner matches the final lineage owner.
8. Event identifiers are unique.
9. Ownership roots evolve across Epoch 0, Epoch 1, and Epoch 2.
10. Replay reproduces the same ownership continuity root.

## PASS Criteria

The certification passes when:

1. Runtime bootstrap certification is available and passing.
2. Asset genesis creates deterministic unique assets.
3. Ownership assignment gives every asset exactly one owner.
4. Transfers preserve asset possession and lineage.
5. Ownership roots evolve across the required epochs.
6. Checkpoint creation and verification pass.
7. Restoration recovers ownership state and lineage.
8. Continued activity advances ownership continuity.
9. Replay produces an identical ownership root.
10. Integrity validation confirms no duplicates, orphans, ambiguity, mismatches, or replay divergence.

## FAIL Criteria

The certification fails if any required status is not `PASS`, including:

- Runtime bootstrap failure.
- Duplicate asset identifiers.
- Missing asset genesis.
- Missing owner after assignment.
- Multiple or ambiguous ownership claims.
- Transfer source mismatch acceptance.
- Ownership loss during transfer.
- Duplicate lineage events.
- Non-evolving epoch roots.
- Checkpoint creation or verification failure.
- Restoration mismatch.
- Replay ownership divergence.

## Relationship To Economic Ledger Certification

Economic Ledger Certification proves balances, supply integrity, transfers, and ledger continuity. Asset Ownership Continuity Certification proves possession, ownership lineage, transfer history, and asset continuity.

Balances and possession are separate runtime concerns. A ledger balance may represent fungible quantity, while asset ownership represents a unique object with one canonical current owner.

## Relationship To Future Vault Ownership Certification

This certification does not prove custody or external authority. Future Vault Ownership Certification can build on this deterministic lineage by adding custody rules, wallet authority, Xaman signatures, XRPL settlement, vault policies, marketplace execution, and external identity validation.

The ownership continuity root produced here is the runtime prerequisite for those later authority and settlement certifications.
