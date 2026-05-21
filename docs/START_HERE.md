# START HERE: Zero-Friction EverArcade Bootstrap

## 1) Run this first

```bash
git clone <repo-url>
cd everarcade-compiler
./scripts/everarcade_start.sh
```

That single command validates environment, vendors dependencies if needed, builds binaries, boots a local federation, generates replay artifacts, and runs simulation inspection.

## 2) What artifacts to inspect

After bootstrap completes, inspect `.everarcade-dev/`:

- `replay.log` — replay execution + convergence marker.
- `simulation.inspect` — simulation inspection output.
- `federation/node-a`, `federation/node-b`, `federation/node-c` — local multi-node federation directories.

## 3) Next commands

```bash
cargo run -p everarcade-cli -- run-local-federation
cargo run -p everarcade-cli -- replay-world
cargo run -p everarcade-cli -- inspect-simulation
```

Optional quick diagnostics:

```bash
./scripts/doctor_quick.sh
cargo run -p everarcade-cli -- doctor
```
