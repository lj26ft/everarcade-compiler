# EverArcade Runtime ↔ HotPocket Integration Proof v0.1

This proof replaces the toy HotPocket gameplay proof with the actual `everarcade-runtime` stack.
HotPocket client inputs are normalized by `adapter/runtime-adapter.js`, passed into the Rust
EverArcade runtime operator command `hotpocket-runtime-proof`, and executed through the runtime
world tick, receipt, journal, checkpoint, replay, restore, and state-root code paths.

## Flow

Client Input → HotPocket Consensus → Adapter → EverArcade Runtime → World Tick → Receipt → Journal → Checkpoint → State Root → Client Output

## Actions

```json
{ "action": "join_player", "player_id": "alice" }
```

```json
{ "action": "move_player", "player_id": "alice", "x": 10, "y": 20 }
```

The adapter only translates action names into runtime arena inputs. State mutation occurs inside
`everarcade-runtime` via `RuntimeLoop::execute_hotpocket_runtime_actions` and `ArenaState::apply`.

## Validation

```bash
node runtime/hotpocket-runtime-proof/validation/hotpocket-runtime-proof.js validate
```

The validator creates reports under `runtime/hotpocket-runtime-proof/reports/` and mirrors them to
repo-level `reports/` for certification tooling.

## Certification

```bash
bash scripts/certify_hotpocket_runtime_integration.sh
```

Expected final line:

```text
EverArcade Runtime ↔ HotPocket Integration Proof v0.1: PASS
```
