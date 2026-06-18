# EverArcade

**Build Worlds. Not Just Games.**

EverArcade is an open-source deterministic game-runtime and creator-tooling repository. It focuses on packaging worlds, running them locally, recording execution evidence, and replay-verifying deterministic outcomes.

EverArcade is currently an **open-source candidate** with a proven local developer onboarding path. It is **not production ready**, **not public-testnet ready**, and **not commercial ready**.

## Why does EverArcade exist?

Games and persistent worlds are hard to audit after they run. EverArcade explores a runtime model where world packages produce receipts, journals, checkpoints, and replay evidence so execution can be inspected and verified later.

The current goal is clarity and local proof, not live multiplayer federation, public settlement, commercial hosting, or production marketplace operation.

## What can I do today?

You can:

- create a local game from the Creator SDK;
- build and validate its manifest;
- package a local runtime bundle;
- run a playable local Arena session;
- inspect generated receipts, journals, state, transcript, and replay proof material;
- use the documentation to understand which subsystems are alpha, experimental, scaffold, or planned.

Treat renderer, history, federation, XRPL settlement, Xahau hooks, GPU marketplace, public testnet, and commercial revenue areas as scaffold or experimental unless `MATURITY.md` says otherwise.

## Quick start

Prerequisites:

- Node.js 18+
- Rust/Cargo
- Network access until offline vendor artifacts are fully restored

Run the targeted developer onboarding validation:

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

Manual local flow:

```bash
TMPDIR="$(mktemp -d)"
PROJECT="$TMPDIR/arena-demo"
RUNTIME_ROOT="$TMPDIR/runtime-root"

node creator-sdk/cli/everarcade.mjs new --template arena --name arena-demo --dir "$PROJECT"
node creator-sdk/cli/everarcade.mjs build --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT"
```

Expected result:

```text
Playable Local Game: PASS
```

## Where are the docs?

- Documentation root: [`docs/index.md`](docs/index.md)
- Documentation policy: [`docs/DOCUMENTATION_POLICY.md`](docs/DOCUMENTATION_POLICY.md)
- Maturity classifications: [`MATURITY.md`](MATURITY.md)
- Repository map: [`REPOSITORY_MAP.md`](REPOSITORY_MAP.md)
- Contributor guide: [`docs/contributor-guide/index.md`](docs/contributor-guide/index.md)
- Open-source readiness audit: [`OPEN_SOURCE_READINESS.md`](OPEN_SOURCE_READINESS.md)
- v0.1 public release readiness: [`docs/open-source/v0.1-public-release-readiness.md`](docs/open-source/v0.1-public-release-readiness.md)

## Contributing

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md), then read the contributor guide under `docs/`. Prefer targeted validation scripts over full workspace test runs unless a change requires broader validation.

Useful checks:

```bash
git diff --check
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash scripts/validate_open_source_readiness.sh
```

Do not claim production readiness from local PASS reports. PASS means the named local proof succeeded under the documented local conditions.
