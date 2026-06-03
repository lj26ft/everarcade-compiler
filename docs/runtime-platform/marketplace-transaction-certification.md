# Marketplace Transaction Certification v0.1

## Purpose

Marketplace Transaction Certification proves that the EverArcade Runtime Platform can execute deterministic, runtime-local marketplace transactions without XRPL, Xaman, hooks, external payment rails, or blockchain settlement.

The certification validates that offer creation, validation, authority checks, acceptance, settlement, ownership transfer, inventory updates, vault custody updates, checkpointing, restoration, replay, and integrity checks produce a stable marketplace continuity root.

## Offer Model

An offer is a deterministic runtime record with:

- Offer identifier.
- Seller.
- Asset.
- Price in Gold.
- Buyer, once accepted.
- Status.
- Settlement marker.

Offer identifiers are fixed for the certification lifecycle. Duplicate offer identifiers are rejected.

## Validation Model

An active offer is valid only when:

- The asset exists.
- The seller owns the asset.
- The seller is the current asset authority.
- The asset inventory matches the seller inventory.
- The asset vault matches the seller vault.
- The price is positive.

Invalid or unauthorized offers are rejected before acceptance or settlement.

## Settlement Model

Settlement is runtime-local and deterministic. The buyer transfers the offer price in Gold to the seller. The certification validates balance conservation, sufficient buyer balance, no negative balances, no inflation, no loss, and double-settlement rejection.

## Ownership Transfer Model

After settlement, ownership transfers from seller to buyer. The asset authority moves with ownership, and the asset lineage appends the buyer while preserving the genesis lineage prefix.

## Inventory Update Model

After ownership transfer, the asset inventory moves to the buyer inventory. The integrity check requires every asset inventory to match the current owner.

## Vault Update Model

After inventory update, vault custody moves to the buyer vault. The integrity check requires every asset vault to match the current owner and prohibits vault custody ambiguity.

## Checkpoint Model

The certification writes a deterministic checkpoint containing:

- Certification version.
- Marketplace checkpoint identifier.
- Persisted marketplace root.
- Canonical marketplace state transcript.

The checkpoint identifier is derived from the checkpoint root and event log hash.

## Replay Model

Replay executes the full lifecycle again from genesis through continued marketplace activity. Replay passes only when the replay marketplace root exactly equals the primary marketplace continuity root.

## Marketplace Integrity Rules

The certification rejects or fails on:

- Duplicate offer identifiers.
- Double settlement.
- Unauthorized sale attempts.
- Cancelled offer settlement.
- Negative balances.
- Balance supply mismatch.
- Ownership ambiguity.
- Inventory mismatch.
- Vault mismatch.
- Replay divergence.

## PASS Criteria

The certification passes when:

1. Runtime bootstrap passes.
2. Marketplace genesis is deterministic.
3. Economic setup preserves non-negative balances.
4. Offer creation and validation succeed.
5. Unauthorized marketplace action is rejected.
6. Offer acceptance succeeds.
7. Settlement conserves balances.
8. Ownership lineage is preserved.
9. Inventory and vault custody update to the buyer.
10. Cancelled offers cannot settle.
11. Marketplace roots evolve across epochs.
12. Checkpoint creation and verification pass.
13. Restoration verifies the checkpoint root.
14. Continued activity produces a continuity root.
15. Replay reproduces the identical continuity root.
16. Marketplace integrity validation passes.

## FAIL Criteria

The certification fails if any required status is not `PASS`, if replay diverges, if any deterministic root is missing, or if any prohibited marketplace condition is accepted.

## Relationship To Wallet Authority Certification

Wallet Authority Certification proves deterministic runtime authority assignment, delegation, revocation, and unauthorized-action rejection. Marketplace Transaction Certification consumes the same authority principle by requiring the seller to be the current asset authority before offer validation, acceptance, settlement, cancellation, or transfer.

This certification does not perform real wallet signatures.

## Relationship To Future XRPL Settlement Certification

Marketplace Transaction Certification intentionally proves only runtime-local settlement. Future XRPL settlement certification can use this as the pre-blockchain baseline for offer lifecycle, economic integrity, ownership continuity, inventory continuity, vault custody, checkpointing, restoration, and replay.

XRPL payment execution, Xaman signing, hooks, federated settlement, and cross-chain settlement remain out of scope for this certification.
