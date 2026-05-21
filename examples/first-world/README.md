# First World (Deterministic Onboarding Demo)

`first-world` is the onboarding world designed to prove the runtime works before reading deep architecture docs.

## What this demonstrates

- **Local federation boot**: creates a 3-node local federation (`node-a/node-b/node-c`).
- **Replay artifacts**: produces `.everarcade-dev/replay.log` from the replay pipeline.
- **Deterministic ticks**: `timelines/world.timeline` records deterministic tick progression.
- **Interaction replay**: replay command reconstructs the deterministic sequence from generated artifacts.
- **Convergence verification**: replay and inspect outputs provide convergence markers for quick verification.

## Fast path

Run:

```bash
./scripts/everarcade_start.sh
```

Then inspect:

- `.everarcade-dev/replay.log`
- `.everarcade-dev/simulation.inspect`
- `.everarcade-dev/timelines/world.timeline`

## Useful commands

```bash
cargo run -p everarcade-cli -- run-local-federation
cargo run -p everarcade-cli -- replay-world
cargo run -p everarcade-cli -- inspect-simulation
cargo run -p everarcade-cli -- reset
```
