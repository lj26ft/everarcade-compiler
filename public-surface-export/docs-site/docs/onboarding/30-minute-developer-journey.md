# 30-Minute Developer Journey

Goal: reach **Playable Local Game Proven** from a fresh checkout without direct maintainer help.

## Prerequisites

- Node.js 18 or newer.
- Rust and Cargo.
- Network access may be needed until the vendored Cargo snapshot is restored; see `docs/build/offline-build-policy.md`.

## 0-5 minutes: clone and inspect

```bash
git clone <repo-url> everarcade-compiler
cd everarcade-compiler
node --version
cargo --version
```

Read the root `README.md` and `docs/repository/repository-map.md` before changing code. The canonical local proof path is Creator SDK → runtime package → local session → replay verification.

## 5-10 minutes: create a game

```bash
TMPDIR="$(mktemp -d)"
PROJECT="$TMPDIR/arena-demo"
node creator-sdk/cli/everarcade.mjs new --template arena --name arena-demo --dir "$PROJECT"
```

Expected output includes:

```text
Project: PASS (arena-demo)
```

## 10-15 minutes: build and validate the project

```bash
node creator-sdk/cli/everarcade.mjs build --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT"
```

Expected output includes:

```text
Build: PASS (arena-demo)
Test: PASS (arena-demo)
```

## 15-20 minutes: package the game

```bash
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT"
```

Inspect the generated runtime package:

```bash
find "$PROJECT/dist/runtime-package" -maxdepth 1 -type f -print | sort
```

Expected files:

- `manifest.json`
- `world.json`
- `world.wasm`

## 20-25 minutes: run a local session and observe gameplay

```bash
RUNTIME_ROOT="$TMPDIR/runtime-root"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT"
```

Expected output:

```text
Playable Local Game: PASS
```

Inspect the local evidence:

```bash
find "$RUNTIME_ROOT" -maxdepth 2 -type f | sort
```

You should see session, gameplay, receipt, journal, transcript, and replay proof files.

## 25-30 minutes: inspect replay proof

```bash
cat "$RUNTIME_ROOT/replay/gameplay-replay-proof.json"
```

Confirm that:

- `replay_verification` is `PASS`.
- `session_id` is `session-0001`.
- `replay_root` matches the final `state_root`.

## One-command validation

After the manual path, run the maintained validation script:

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

Required report:

```text
reports/developer_onboarding_validation_report.txt
```

The report must show PASS for repository bootstrap, developer build, Creator SDK, runtime package, playable local game, and replay verification.
