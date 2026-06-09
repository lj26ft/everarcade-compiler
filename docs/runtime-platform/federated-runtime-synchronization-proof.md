# Federated Runtime Synchronization Proof v0.1

## Classification

**Federated Runtime Synchronization Proven**

This proof demonstrates local synchronization between two independent authoritative EverArcade runtime instances through deterministic evidence exchange. It extends the previous `Network Transport & Session Synchronization Proven` milestone from multiple clients on one runtime to multiple authoritative runtime identities in one local federation.

## What this proves

This proves local federation synchronization.

Two independent runtime authorities are created:

- **Runtime A** owns the session, executes Arena gameplay, and generates receipts, journal entries, checkpoints, and replay evidence.
- **Runtime B** starts as a separate authority identity, imports Runtime A evidence, validates the checkpoint, receipts, and journal, synchronizes state, detects invalid evidence, and recovers from behind-state by checkpoint import plus receipt replay.

Runtime B does **not** share Runtime A memory. It accepts synchronization only through generated evidence files under the local federation evidence directory.

## What this does not prove

It does not prove:

- Evernode deployment,
- WAN federation,
- XRPL settlement,
- production-scale federation,
- or multi-lease gameplay hosting.

It also does not prove public testnet operation, marketplace integration, GPU execution, commercial revenue, or blockchain settlement.

## Federation identities

Each runtime identity is deterministic and contains:

- `runtime_id`
- `federation_id`
- `epoch`
- `authority_root`

The proof requires `Runtime A != Runtime B` while both runtimes share the same `federation_id`, `epoch`, and `authority_root`. This establishes two distinct local authorities participating in one local federation epoch.

## Authority model

Runtime A is authoritative for gameplay execution in this proof. Runtime B is independently authoritative for validation and synchronization, but it does not mutate gameplay locally during the federated gameplay sync. Instead, it verifies Runtime A evidence and imports the accepted state root.

The authority root is a deterministic commitment over the local federation policy, epoch, and Arena template. It is local proof evidence only; it is not an Evernode, XRPL, or production federation anchor.

## Checkpoint exchange

The proof generates:

- `checkpoint-a.json`
- `checkpoint-b.json`
- `checkpoint-exchange.json`

Runtime A sends its checkpoint to Runtime B. Runtime B validates federation identity, epoch, authority root, state root, receipt root, journal root, and checkpoint root before accepting the checkpoint. The exchange evidence records:

- checkpoint verified,
- checkpoint accepted,
- checkpoint imported.

A reverse Runtime B to Runtime A checkpoint is also generated to prove two-runtime evidence exchange exists, even though Runtime A remains the gameplay executor for this local scenario.

## Receipt exchange

The proof generates:

- `receipt-stream-a.jsonl`
- `receipt-stream-b.jsonl`
- `receipt-exchange.json`

Runtime A receipts commit to each gameplay action, tick, player, state root, and receipt root. Runtime B validates the transferred receipt stream against Runtime A's checkpoint receipt root. Runtime B imports receipts as evidence and records that no local gameplay mutation occurred during receipt import.

## Journal exchange

The proof generates:

- `journal-a.jsonl`
- `journal-b.jsonl`
- `journal-exchange.json`

Journal entries contain deterministic observations of intent hashes, actions, receipt hashes, and state roots. Runtime B verifies the journal root against Runtime A's checkpoint and confirms that every journal entry corresponds to an observed receipt.

## Synchronization model

The deterministic synchronization flow is:

1. Runtime A executes Arena gameplay.
2. Runtime A emits checkpoint, receipt stream, journal stream, and replay evidence.
3. Runtime B imports Runtime A checkpoint evidence.
4. Runtime B verifies receipt and journal roots against the checkpoint.
5. Runtime B synchronizes its state root to the accepted Runtime A root.
6. The proof writes `sync-epoch.json` and `sync-state.json`.

Synchronization passes only when:

```text
Runtime A root = Runtime B root
```

## Federated gameplay model

The local Arena gameplay sequence is:

```text
Join
Move
Attack
Score Update
```

Runtime A executes the sequence and produces authoritative evidence. Runtime B synchronizes from evidence without re-executing gameplay locally. The proof writes `federated-gameplay-proof.json` with `gameplay_state_synchronized: true` and `reexecuted_locally: false`.

## Divergence model

The divergence scenario tampers with Runtime A checkpoint state before Runtime B import. Runtime B recomputes the checkpoint commitment and rejects the invalid checkpoint.

The proof writes `divergence-proof.json` and requires:

- divergence detected,
- checkpoint rejected,
- synchronization halted.

## Recovery model

The recovery scenario starts Runtime B behind Runtime A. Recovery proceeds by:

1. checkpoint import,
2. receipt replay,
3. state reconstruction.

The proof writes `recovery-proof.json` and requires:

- recovery successful,
- state restored.

## Replay model

Replay evidence is written to:

```text
replay/federation-replay-proof.json
```

Replay verification passes only when:

```text
Runtime A replay root = Runtime B replay root = Federation root
```

The expected result is:

```text
Federation Replay Verification: PASS
```

## Creator SDK command

Run the local federated proof with:

```bash
node creator-sdk/cli/everarcade.mjs play-federated-local --template arena
```

The command launches Runtime A and Runtime B proof identities, executes gameplay on Runtime A, exchanges checkpoints, receipts, and journals, synchronizes Runtime B, verifies replay, and generates local evidence.

Expected output:

```text
Federated Runtime Synchronization: PASS
```

## Validation and certification

Validation:

```bash
bash scripts/validate_federated_runtime_sync.sh
```

Certification:

```bash
bash scripts/certify_federated_runtime_sync.sh
```

The validation report is written to `reports/federated_runtime_validation_report.txt`. The certification report is written to `reports/federated_runtime_certification_report.txt` and ends with:

```text
Federated Runtime Synchronization Proof v0.1: PASS
```

## Limitations

This is a deterministic local proof. Evidence exchange occurs through local files, not WAN transport. Authority identity is deterministic local proof identity, not production federation identity. No Evernode lease, XRPL transaction, public testnet, settlement layer, marketplace, GPU provider, or production hosting path is exercised.
