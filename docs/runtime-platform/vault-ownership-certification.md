> **Repository boundary:** This repository does not implement custody, wallet management, live settlement, or legal ownership certification.
>
> XRPL/Xahau material is limited to deterministic anchoring, boundary modeling, or local test scaffolds unless explicitly stated otherwise.
>
# Vault Ownership Certification v0.1

## Purpose

Vault Ownership Certification proves that the EverArcade Runtime Platform can maintain deterministic vault custody across asset genesis, ownership assignment, inventory assignment, vault creation, deposits, authorized custody transfers, withdrawals, checkpointing, restoration, continued activity, and replay.

This certification validates runtime custody only. It does not validate wallet authority, Xaman signatures, XRPL ownership, XRPL settlement, hooks, multisig, marketplace execution, federation settlement, or external identity.

## Vault Model

The certification creates four deterministic assets:

- `asset:sword-001` (`Sword #001`).
- `asset:shield-001` (`Shield #001`).
- `asset:potion-001` (`Potion #001`).
- `asset:gem-001` (`Gem #001`).

The deterministic vaults are:

- `vault:a` (`Vault A`).
- `vault:b` (`Vault B`).
- `vault:treasury` (`Treasury Vault`).

Vault roots are SHA-256 hashes over a canonical transcript containing the certification version, authority boundary, fixed asset order, owner order, inventory order, vault order, vault creation state, vault membership counts, asset owners, inventory membership, vault custody slots, lineage, and deterministic event log.

## Custody Model

Vault custody is represented by one asset-to-vault slot. An asset in vault custody must not also be in active inventory.

Vaults represent runtime custody. Vaults do not represent wallet authority, XRPL ownership, Xaman signatures, external settlement, or external identity.

## Ownership Relationship

Ownership remains a separate runtime possession layer. Depositing an asset into a vault changes custody, not owner.

The certification requires each asset to have exactly one owner after ownership assignment. Withdrawals return assets only to inventories whose deterministic owner matches the asset owner, preserving the inventory ownership rule.

## Deposit Model

A deposit moves an asset from active inventory into a created vault.

A valid deposit requires:

1. Known asset.
2. Known source inventory.
3. Created destination vault.
4. Source inventory owner matching the asset owner.
5. Asset currently present in the source inventory.
6. Asset not already in vault custody.
7. Custody lineage appended.

After deposit, the asset inventory slot is empty and the asset vault slot is populated.

## Transfer Model

A vault transfer moves custody from one vault to another without changing owner.

The certification uses deterministic authorization rules for allowed custody routes:

- `vault:a` to `vault:b`.
- `vault:b` to `vault:treasury`.
- `vault:treasury` to `vault:b`.

Unauthorized routes are rejected and must not mutate state.

## Withdrawal Model

A withdrawal removes custody and returns an asset to an owner-compatible active inventory.

A valid withdrawal requires a known asset, known source vault, known destination inventory, current custody in the source vault, no active inventory membership, and destination inventory ownership matching the asset owner.

After withdrawal, the asset vault slot is empty and the asset inventory slot is populated.

## Checkpoint Model

After deterministic vault evolution, the certification writes a vault checkpoint containing:

- Checkpoint version.
- Vault checkpoint identifier.
- Persisted vault root.
- Canonical vault transcript.

The checkpoint identifier is derived from the certification version, persisted vault root, and event log hash. Checkpoint verification requires the persisted identifier and root to match the canonical state.

## Replay Model

Replay independently executes the complete lifecycle:

```text
Genesis -> Ownership -> Inventory -> Vault Creation -> Deposit -> Transfer -> Withdrawal -> Evolution -> Checkpoint -> Restart -> Restore -> Continue Activity
```

Replay passes only when the replay vault root exactly matches the primary vault continuity root. A mismatch indicates nondeterministic custody ordering, checkpoint drift, restoration drift, lineage drift, or hidden vault membership ambiguity.

## Custody Integrity Rules

The certification requires:

1. Every expected asset is created exactly once.
2. Every asset has exactly one runtime owner after ownership assignment.
3. Every vault identifier is unique.
4. Every vault entry references a known created vault.
5. Every vaulted asset exists exactly once.
6. No asset is both vaulted and actively inventoried.
7. No duplicate vault membership exists.
8. No orphaned vault entry exists.
9. Inventory membership matches owner when an asset is not vaulted.
10. Event identifiers are unique.
11. Lineage begins at genesis and records ownership, inventory, deposit, custody transfer, and withdrawal events.
12. Vault roots evolve across Epoch 0, Epoch 1, and Epoch 2.
13. Replay reproduces the same vault continuity root.

## PASS Criteria

The certification passes when:

1. Runtime bootstrap certification is available and passing.
2. Asset genesis creates deterministic unique assets.
3. Ownership assignment gives every asset exactly one owner.
4. Inventory assignment gives every non-vaulted asset an owner-compatible inventory.
5. Vault creation produces unique vault identifiers.
6. Deposits remove assets from active inventory and add vault custody.
7. Authorized transfers preserve custody continuity.
8. Unauthorized transfers are rejected.
9. Withdrawals remove custody and restore inventory membership without corrupting ownership.
10. Checkpoint creation and verification pass.
11. Restoration recovers vault state, custody, and membership.
12. Continued activity advances vault continuity.
13. Replay produces an identical vault root.
14. Custody integrity confirms no duplicates, missing custody, orphaned entries, ownership corruption, lineage corruption, or replay divergence.

## FAIL Criteria

The certification fails if any required status is not `PASS`, including:

- Runtime bootstrap failure.
- Duplicate asset identifiers.
- Duplicate vault identifiers.
- Missing owner after assignment.
- Ownership and inventory-owner mismatch.
- Duplicate custody.
- Missing custody.
- Orphaned vault entries.
- Invalid deposit acceptance.
- Unauthorized transfer acceptance.
- Invalid withdrawal acceptance.
- Lineage corruption.
- Non-evolving epoch roots.
- Checkpoint creation or verification failure.
- Restoration mismatch.
- Replay vault divergence.

## Relationship To Inventory Certification

Inventory Continuity Certification proves deterministic runtime collection membership for owned assets. Vault Ownership Certification builds on that layer by proving that an asset can leave active inventory, enter custody, move between vaults, leave custody, and replay to the same custody root.

Inventory answers where an owned asset lives in active collection state. Vault custody answers which runtime vault currently holds custody when the asset is not active in inventory.

## Relationship To Future Wallet Authority Certification

This certification intentionally excludes wallet authority and external signatures. Future Wallet Authority Certification can use these deterministic vault custody roots as inputs while adding wallet policies, Xaman signatures, XRPL settlement, hooks, multisig, marketplace execution, federation settlement, and external identity validation.

The vault continuity root produced here is the runtime custody prerequisite for those later authority and settlement certifications.
