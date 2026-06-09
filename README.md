# EverArcade

EverArcade is a deterministic game-runtime and creator-tooling repository for packaging, running, and auditing local game execution. The current repository proves that a template game can be created, packaged, executed by the local runtime, and replay-verified on a developer machine.

EverArcade is **not production ready**, **not public-testnet ready**, and **not commercial ready**. The current classification is:

```text
Developer Onboarding Proven
Open Source Candidate
```

## Project Overview

EverArcade exists to make game execution reproducible enough that runtime state, receipts, journals, and replay evidence can be inspected after a session. The repository combines:

- a Rust runtime prototype in `runtime/everarcade-runtime/`;
- a Node-based Creator SDK in `creator-sdk/`;
- template games and sample packages;
- validation and certification scripts under `scripts/`;
- reports and documentation that explain which claims are proven and which are only scaffolded.

The immediate purpose of v0.1 is developer comprehension and local proof, not live multiplayer, settlement, marketplace operation, or production hosting.

## Architecture Overview

At a high level, EverArcade has four practical layers:

1. **Creator layer**: `creator-sdk/` creates a game project, validates its manifest, builds local artifacts, and packages a runtime bundle.
2. **Runtime package layer**: generated packages contain `manifest.json`, `world.json`, and `world.wasm` or a deterministic placeholder world artifact.
3. **Local runtime layer**: `runtime/everarcade-runtime/` starts local sessions, writes receipts, journals, gameplay state, and replay proof material.
4. **Evidence layer**: `reports/`, runtime roots, and generated validation outputs document what happened and whether replay verification passed.

Many other directories are intentionally less mature. Treat renderer, history, federation, marketplace, GPU, XRPL, Xaman, public-testnet, and commercial-revenue areas as scaffold-level or experimental unless a current validation script and proof-chain document say otherwise. See `docs/repository/repository-map.md` for subsystem ownership and maturity.

## Current Status

### What currently works

- Creating a local Arena game from the Creator SDK.
- Building and validating the Creator SDK project manifest.
- Packaging a local runtime package.
- Running a playable local Arena session through `everarcade-runtime`.
- Producing local session, gameplay, receipt, journal, transcript, and replay verification evidence.
- Running open-source readiness and developer-experience certification scripts.

### What does not work yet

- Production hosting or public testnet operation.
- Real multiplayer federation gameplay.
- Renderer-driven end-user gameplay as the canonical proof path.
- XRPL settlement, Xaman signing, GPU marketplace, commercial revenue, and marketplace workflows as production systems.
- A fully resolved offline vendor snapshot in this clone; the known missing `bincode` vendor issue is documented in `docs/build/offline-build-policy.md`.

## Quick Start

Prerequisites:

- Node.js 18+ for the Creator SDK CLI.
- Rust/Cargo for the local runtime proof.
- Network access may be required until the vendor snapshot is restored.

Run the developer onboarding validation:

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

Run the open-source readiness audit:

```bash
bash scripts/validate_open_source_readiness.sh
```

Run developer-experience certification:

```bash
bash scripts/certify_developer_experience.sh
```

## Playable Local Game

Manual flow:

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

The runtime root should contain session state, gameplay state, receipts, a journal stream, a session transcript, and `replay/gameplay-replay-proof.json` with replay verification set to `PASS`.

For a guided 30-minute path, use `docs/onboarding/30-minute-developer-journey.md`.

## Roadmap

Near-term roadmap:

1. Keep the root README, repository map, proof chain, artifact policy, and onboarding guide as canonical entry points.
2. Consolidate duplicate scripts and proof reports without deleting historical evidence prematurely.
3. Restore a complete vendor artifact and document the supported offline build process.
4. Expand renderer-driven local gameplay only after the current CLI-based playable local proof remains stable.
5. Continue labeling federation, marketplace, XRPL, Xaman, GPU, and commercial-revenue domains honestly as scaffold or experimental until proven otherwise.

## Contributing

Start with `CONTRIBUTING.md`, then run:

```bash
git diff --check
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash scripts/validate_open_source_readiness.sh
bash scripts/certify_developer_experience.sh
```

Do not claim production readiness from local PASS reports. PASS means the named local proof succeeded under the documented conditions.
