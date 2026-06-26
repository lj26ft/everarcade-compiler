# EverArcade

**Build Worlds. Not Just Games.**

EverArcade is an open-source deterministic game-runtime and creator-tooling repository. It focuses on packaging worlds, running them locally, recording execution evidence, and replay-verifying deterministic outcomes.

EverArcade is currently an **open-source candidate** with a proven local developer onboarding path. It is **not production ready**, **not public-testnet ready**, and **not commercial ready**.

## Why does EverArcade exist?

Games and persistent worlds are hard to audit after they run. EverArcade explores a runtime model where world packages produce receipts, journals, checkpoints, and replay evidence so execution can be inspected and verified later.

The current goal is clarity and local proof, not live multiplayer federation, public settlement, commercial hosting, or production marketplace operation.

## What can I do today?

You can:

- create a local World from the Creator SDK;
- discover World templates and RustRigs;
- build and validate its metadata;
- package a local World Package;
- run a playable local World session;
- inspect generated receipts, journals, state, transcript, and replay proof material;
- use the documentation to understand which subsystems are alpha, experimental, scaffold, or planned.

Treat renderer, history, federation, XRPL settlement, Xahau hooks, GPU marketplace, public testnet, and commercial revenue areas as scaffold or experimental unless `MATURITY.md` says otherwise.

## Day 1 contributor (canonical gate)

Prerequisites: Node.js 18+, Rust/Cargo. **No network required after clone** — dependencies ship in `dist/vendor.tar.gz`.

```bash
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Expected: `Prerequisites: PASS`, onboarding PASS, and `REFERENCE CERTIFIED WORLD V1: PASS`. CI enforces the same gate offline on Ubuntu and macOS.

Offline vendor policy: [`docs/build/offline-build-policy.md`](docs/build/offline-build-policy.md)

## RC2 Independent Reviewer Path

Use this single path to reproduce the authoritative RC2 review target from a fresh clone:

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
git checkout 53a17567e826c5d4f9b083e490cf1568bfe7534e

scripts/ci/check-rc2-commit-pins.sh

# Run the normal RC2 gate.
CARGO_BUILD_JOBS=1 scripts/ci/rc2-gate.sh

# Run must-fail fixture: self-attested fork must fail.
scripts/ci/rc2-fixture-self-attested-fork-must-fail.sh

# Run must-fail fixture: tampered payload must fail.
scripts/ci/rc2-fixture-tampered-payload-must-fail.sh
```

Expected result:

- The normal RC2 gate passes.
- The commit-pin consistency check passes.
- The self-attested-fork fixture fails verification.
- The tampered-payload fixture fails verification.
- If either must-fail fixture passes, RC2 is not valid.

## Quick start

Manual First World flow:

```bash
TMPDIR="$(mktemp -d)"
PROJECT="$TMPDIR/frontier-world"

node creator-sdk/cli/everarcade.mjs world templates
node creator-sdk/cli/everarcade.mjs world rustrigs
node creator-sdk/cli/everarcade.mjs world init --template frontier --name frontier-world --dir "$PROJECT"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs world run --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world package --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world verify --project "$PROJECT"
```

Expected result:

```text
WORLD VERIFY: PASS
```

Start with [`docs/first-world.md`](docs/first-world.md) and the generated project map in [`docs/creator-sdk/world-project-map.md`](docs/creator-sdk/world-project-map.md).

## Where are the docs?

- Architecture overview: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- Compiler documentation root: [`docs/index.md`](docs/index.md)
- Public websites, `docs.everarcade.games`, `vision.everarcade.games`, and the world portal live in `everarcade-frontend`. See [`docs/public-frontend-surface.md`](docs/public-frontend-surface.md).
- Documentation policy: [`docs/DOCUMENTATION_POLICY.md`](docs/DOCUMENTATION_POLICY.md)
- Maturity classifications: [`MATURITY.md`](MATURITY.md)
- Repository map: [`REPOSITORY_MAP.md`](REPOSITORY_MAP.md)
- Source/artifact and reproducibility policy: [`REPOSITORY_POLICY.md`](REPOSITORY_POLICY.md)
- Script manifest: [`scripts/MANIFEST.md`](scripts/MANIFEST.md)
- Contributor guide: [`docs/contributor-guide/index.md`](docs/contributor-guide/index.md)
- Open-source readiness audit: [`OPEN_SOURCE_READINESS.md`](OPEN_SOURCE_READINESS.md)
- v0.1 public release readiness: [`docs/open-source/v0.1-public-release-readiness.md`](docs/open-source/v0.1-public-release-readiness.md)


## Public Websites and Portal UI

Public websites, `docs.everarcade.games`, `vision.everarcade.games`, and the world portal live in `everarcade-frontend`. This compiler repository links to those surfaces but does not own their Vercel/Docusaurus/portal presentation.

Placeholders:

- `everarcade-frontend`
- `https://docs.everarcade.games`
- `https://vision.everarcade.games`

## Contributing

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md), then read the contributor guide under `docs/`. Prefer targeted validation scripts over full workspace test runs unless a change requires broader validation.

Useful checks:

```bash
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
bash scripts/validate_open_source_readiness.sh
git diff --check
```

Do not claim production readiness from local PASS reports. PASS means the named local proof succeeded under the documented local conditions.
