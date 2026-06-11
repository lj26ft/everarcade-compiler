# HotPocket Consensus Gameplay Proof v0.1

This proof validates deterministic EverArcade gameplay execution through a HotPocket-compatible consensus harness. It proves that the canonical gameplay model can be applied identically by three validators, emits canonical receipts and journals, verifies client round trips, and replays the persisted action sequence from genesis.

## Gameplay model

Genesis state:

```json
{
  "players": {},
  "tick": 0
}
```

Supported deterministic actions:

- `ping`
- `join_player` with `player_id`
- `move_player` with `player_id`, integer `x`, and integer `y`

The model uses no randomness, no wall-clock time, no external APIs, and no validator-local state.

## Validation

```bash
node runtime/hotpocket-gameplay-proof/validation/hotpocket-gameplay-proof.js validate
```

The command performs cluster discovery, package/deployment validation, gameplay execution, validator agreement checks, state-root verification, client round-trip validation, replay verification, and failure-signature inspection.

## Certification

```bash
bash scripts/certify_hotpocket_gameplay.sh
```

Expected final line:

```text
HotPocket Consensus Gameplay Proof v0.1: PASS
```

## Reports

Canonical gameplay artifacts are written to both `runtime/hotpocket-gameplay-proof/reports/gameplay/` and root `reports/gameplay/` for certification collection:

- `action_sequence.json`
- `join_player_receipt.json`
- `move_player_receipt.json`
- `state_root.txt`
- `execution_journal.json`
- `validator_executions.json`

Top-level proof reports include:

- `gameplay_state_root_report.txt`
- `gameplay_validator_agreement_report.txt`
- `gameplay_roundtrip_report.txt`
- `gameplay_replay_report.txt`
- `gameplay_failure_report.txt`
- `hotpocket_gameplay_validation_report.txt`
