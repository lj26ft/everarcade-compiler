# HotPocket Contract Adapter Proof v0.1

## Scope

This proof establishes HotPocket ↔ EverArcade execution compatibility for the adapter boundary only. It does **not** claim Evernode lease deployment, WAN federation, XRPL or Xahau settlement, civilization hosting, multiplayer at scale, or production economics.

## Architecture

`runtime/hotpocket-adapter` is the canonical adapter package. It accepts HotPocket user input JSON, canonicalizes it, converts it into `everarcade.runtime.input.v0.1`, executes deterministic EverArcade mutations, and persists roots under the configured adapter state directory.

The adapter produces four deterministic artifact classes:

- Receipts: client-visible execution result and root references.
- Journals: before/after mutation records.
- Checkpoints: compact deterministic state snapshots.
- Replay proofs: input, mutation, and checkpoint hashes used to compare repeated runs.

## Deployment

`creator-sdk/templates/hotpocket-adapter` provides a creator-facing contract entrypoint. `scripts/run_hotpocket_package_proof.sh` builds and stages `dist/everarcade-hotpocket-contract` using repository scripts only. No manual container edits, manual `patch.cfg` edits, or manual `bin_path` edits are required by the package proof.

## Consensus

`scripts/run_hotpocket_consensus_proof.sh` exercises the canonical 3-node proof path and validates that input acceptance, finalization, receipt generation, and output return complete without `max_ledger_expired` or `Not enough peers proposing` diagnostics.

## Execution

`scripts/run_hotpocket_execution_proof.sh` submits `{"action":"ping"}` through the adapter contract boundary and requires `{"status":"ok"}`.

## State Mutation

`join_player` is the minimal deterministic mutation. It increments `player_count` by one and generates a receipt, journal entry, checkpoint, and replay proof.

## Replay

`scripts/run_hotpocket_replay_proof.sh` executes an identical input sequence twice and compares `state_root`, `receipt_root`, `journal_root`, and `replay_root`.

## Receipts

Receipts use schema `everarcade.receipt.v0.1` and contain status, action, input hash, mutation hash, output, and current roots.

## Journals

Journal entries use schema `everarcade.journal.v0.1` and record the before root, player count before and after, input hash, and mutation hash.

## Checkpoints

Checkpoints use schema `everarcade.checkpoint.v0.1` and snapshot sequence, player count, and seen input hashes. They are deterministic and feed replay proof hashing.

## Package Loading

The package proof stages deterministic EverArcade runtime packages into `dist/everarcade-hotpocket-contract/packages`, verifies `packages.sha256`, and runs the staged `start.sh` without manual patching.

## SDK Discovery

`runtime/hotpocket-adapter/bin/discover-hotpocket-sdk.js` records installed HotPocket package versions, module export surfaces, callback signature hypotheses, client event hypotheses, consensus environment variables, and runtime metadata to `reports/hotpocket_sdk_discovery_report.json`.

## Limitations

The proof is adapter-level and intentionally excludes settlement, federation, WAN hosting, and production economic assumptions. If no HotPocket SDK package is installed in the local environment, the discovery report records unavailable packages rather than inventing an API surface.

## Certification

Run:

```bash
bash scripts/certify_hotpocket_adapter.sh
```

A successful run ends with:

```text
HotPocket Contract Adapter Proof v0.1: PASS
```
