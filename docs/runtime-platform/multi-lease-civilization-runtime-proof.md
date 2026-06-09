# Multi-Lease Civilization Runtime Proof v0.1

The multi-lease civilization runtime proof demonstrates a local, deterministic civilization that survives across two independent lease-style runtime authorities. It extends the local federation proof from runtime-state synchronization into persistent civilization continuity for population, economy state, inventory state, checkpoint evidence, recovery, and replay.

This proves local multi-lease civilization continuity.

It does not prove:
Evernode deployment,
XRPL settlement,
production civilization hosting,
WAN federation,
or economic finality.

## Civilization model

The proof creates a deterministic civilization genesis record and evolves a civilization state with:

- `population`
- `resources`
- `economy`
- `inventory`
- `territories`
- `civilization_score`

The canonical state root is derived from the civilization identity, population, resources, economy, inventory, territories, score, and the derived economy, inventory, and world roots. This keeps replay comparison independent from process memory and lease-local object identity.

## Lease identities

The proof creates two deterministic lease identities:

- `lease-a`
- `lease-b`

Each lease identity contains:

- `lease_id`
- `runtime_id`
- `federation_id`
- `civilization_id`
- `epoch`
- `authority_root`

Validation requires `lease-a != lease-b` while preserving an identical `civilization_id`, `federation_id`, epoch, and authority root. The leases are independent local authorities, not live Evernode leases.

## Economy continuity

Lease A performs deterministic economy activity:

1. Resource creation.
2. Resource consumption.
3. Economic transfer.
4. Score update.

The proof writes `economy-ledger.jsonl` and `economy-continuity-proof.json`. Validation proves the economy root changes during activity and the final economy root is preserved when Lease B reconstructs state from evidence.

## Inventory continuity

Lease A performs deterministic inventory activity:

1. Item creation.
2. Item transfer.
3. Item ownership verification.

The proof writes `inventory-ledger.jsonl` and `inventory-continuity-proof.json`. Validation proves the item owner remains verifiable after the lease transition and that the inventory root is preserved by Lease B.

## Lease transition

Lease A joins the civilization, mutates economy and inventory state, generates receipts, journals, and a checkpoint, then exports evidence. Lease B imports that evidence and reconstructs the civilization without direct memory sharing.

The transition proof validates:

- Lease A and Lease B are distinct leases.
- The civilization ID is unchanged.
- The continuity root is unchanged across the lease transition.
- Civilization state survives the transition.

## Lease failure

The local proof simulates Lease A becoming unavailable after evidence export. Lease B continues from the imported checkpoint, receipt stream, and journal stream. The failure proof validates that Lease A is unavailable, Lease B continues the same civilization, and no state loss is detected.

## Recovery

The recovery flow follows:

```text
checkpoint import
↓
receipt replay
↓
journal replay
↓
state reconstruction
```

The recovery proof validates that the reconstructed root equals the expected civilization root.

## Replay

The replay proof rebuilds the civilization from genesis plus the deterministic economy and inventory actions. Replay passes only when:

```text
replay root = civilization root
```

Expected terminal output:

```text
Civilization Replay Verification: PASS
Multi-Lease Civilization Runtime: PASS
```

## Evidence layout

The Creator SDK command:

```bash
node creator-sdk/cli/everarcade.mjs play-multi-lease-local --template civilization
```

writes evidence under the selected runtime root:

```text
civilization/
├── genesis.json
├── economy-ledger.jsonl
├── inventory-ledger.jsonl
├── economy-continuity-proof.json
├── inventory-continuity-proof.json
├── checkpoint-exchange.json
├── civilization-sync-proof.json
├── civilization-recovery-proof.json
├── lease-transition-proof.json
├── lease-failure-proof.json
├── summary.json
├── lease-a/
│   ├── identity.json
│   ├── lease-a-state.json
│   ├── receipts.jsonl
│   └── journal.jsonl
├── lease-b/
│   ├── identity.json
│   ├── lease-b-state.json
│   └── continuation-receipt.json
└── replay/
    └── civilization-replay-proof.json
```

Required root fields are present in the proof artifacts:

- `civilization_id`
- `lease_id`
- `epoch`
- `civilization_root`
- `economy_root`
- `inventory_root`

## Validation and certification

Validation:

```bash
bash scripts/validate_multi_lease_civilization.sh
```

Certification:

```bash
bash scripts/certify_multi_lease_civilization.sh
```

The certification final line is:

```text
Multi-Lease Civilization Runtime Proof v0.1: PASS
```

## Limitations

This is a local deterministic proof. It does not interact with XRPL, Xaman, Evernode hosts, public testnets, production lease infrastructure, WAN networking, GPU execution, marketplace flows, or commercial settlement. The economy ledger is a deterministic local continuity ledger, not economic finality or real settlement.
