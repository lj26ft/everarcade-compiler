# Public Determinism Fixtures

These fixtures are public replay vectors for `.evr` deterministic verification. They are intentionally small, hardware-independent, and defined in terms of canonical JSON plus SHA-256 so independent implementations can reproduce the same roots without trusting EverArcade.

## Fixture registry

| Fixture | Class | Purpose | Expected roots |
| --- | --- | --- | --- |
| `basic-counter-v1/inputs.json` | `DETERMINISTIC` | Exercises integer state transition replay with ordered public inputs. | `basic-counter-v1/expected-roots.json` |

## Canonicalization

Fixtures use UTF-8 JSON with sorted object keys, no insignificant whitespace, and exactly one trailing LF. Hashes are lowercase SHA-256 hex strings prefixed with `sha256:`.

## Replay rule for `basic-counter-v1`

Start with `initial_state.counter` and an empty `log`. For each ordered step:

- `add`: `counter = counter + value`
- `mul`: `counter = counter * value`
- `xor`: `counter = counter XOR value`
- `mod`: `counter = counter % value`

Append `{"tick", "action", "value", "counter"}` to `log` after each step. The state root for each tick is the SHA-256 of the canonical JSON state after that step.

## Independent verification

Run from the repository root:

```bash
python3 fixtures/determinism/verify_basic_counter.py
```

A conforming implementation must reproduce every `step_roots[].state_root` and `final_root` exactly.
