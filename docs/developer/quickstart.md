# Local developer quickstart

This path uses the actual Creator SDK arena template and local Rust runtime. Creator SDK and runtime are **ALPHA**; renderer, federation and settlement include scaffold areas. A source `Build: PASS` or `Test: PASS` checks metadata/source only. It does not prove Wasm compilation, native gameplay, production readiness, trusted receipt acceptance, or an independently accepted result. See [maturity](../../MATURITY.md) and [contributor contract](../../CONTRIBUTING.md).

## Clone, pin and prerequisites

Start in a fresh checkout. Keep existing changes and worker branches; do not run destructive checkout commands over them.

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
git fetch origin 33aa49b51d2095d2154049761d4b1734d9f85fc7
git switch -c local/arena-quickstart 33aa49b51d2095d2154049761d4b1734d9f85fc7
```

That immutable source pin defines the CLI/template described here. To use the smoke helper introduced by this PR, check out the reviewed quickstart branch/commit instead, preserving its ancestry from that pin.

Use **Node 24.15.0** (`.nvmrc`), **Rust/Cargo 1.95.0** (`rust-toolchain.toml`), Bash, Git, tar, npm and a SHA256 tool. Python 3.9+ is required only for the bounded helper. Install the pinned tools through your normal tool manager before offline execution; the Rust toolchain also needs `wasm32-unknown-unknown`:

```bash
node --version
rustc --version
cargo --version
rustup target add --toolchain 1.95.0 wasm32-unknown-unknown
rustup target list --installed
bash scripts/ensure_vendor_offline.sh
bash scripts/check_prerequisites.sh
```

The toolchain/target installation can require network; Cargo dependencies come from the committed checksum-verified `dist/vendor.tar.gz`. Never regenerate locks, vendor archives or expected hashes to force a pass. See [offline policy](../build/offline-build-policy.md). Prerequisites restore `vendor/`, run offline Cargo checks, and use `/tmp` logs. Full contributor scripts also write tracked reports; use an isolated checkout and inspect changes.

## Create, build and test

Run commands from the repository root. These variables designate a **new disposable directory**; `build` removes the generated project's `dist`. Preserve the directory until you finish inspecting evidence.

```bash
QUICKSTART_TMP="$(mktemp -d)"
QUICKSTART_PROJECT="$QUICKSTART_TMP/arena-demo"
QUICKSTART_RUNTIME="$QUICKSTART_TMP/runtime-root"
node creator-sdk/cli/everarcade.mjs new --template arena --name arena-demo --dir "$QUICKSTART_PROJECT"
node creator-sdk/cli/everarcade.mjs build --project "$QUICKSTART_PROJECT"
node creator-sdk/cli/everarcade.mjs test --project "$QUICKSTART_PROJECT"
```

Expect `Project: PASS (arena-demo)`, `Build: PASS (arena-demo)`, and `Test: PASS (arena-demo)`. At this point only template copy, source checks and build metadata have passed.

## Package and run locally

The arena template declares `contracts/arena-proof-contract` as its guest. Packaging builds a real Wasm module. Upstream packaging deletes `/tmp/everarcade-guest-build/arena-proof-contract` before rebuilding: preserve any existing directory there and use an isolated environment if it belongs to other work. Do not run package jobs concurrently. The CLI will try installing the Wasm target if absent; install it explicitly above before offline use.

```bash
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs package --project "$QUICKSTART_PROJECT"
find "$QUICKSTART_PROJECT/dist/runtime-package" -maxdepth 1 -type f -print
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs play-local --project "$QUICKSTART_PROJECT" --template arena --runtime-root "$QUICKSTART_RUNTIME"
```

Expect `manifest.json`, `world.json`, and `world.wasm`; `world.json` should identify `wasm-guest-runtime-package`. The native command invokes repo-root `cargo run --offline --locked -p everarcade-runtime ... local-session` and must print `Playable Local Game: PASS`. Local-session generates deterministic session/gameplay/replay evidence; it is not an interactive renderer or a deployed service.

## Inspect replay

`play-local` performs the local replay verification; there is no extra invented replay command in this path.

```bash
node --input-type=module - "$QUICKSTART_RUNTIME" <<'JS'
import fs from 'node:fs';
import assert from 'node:assert/strict';
const root = process.argv[2];
const proof = JSON.parse(fs.readFileSync(`${root}/replay/gameplay-replay-proof.json`, 'utf8'));
assert.equal(proof.replay_verification, 'PASS');
assert.equal(proof.session_id, 'session-0001');
assert.ok(proof.replay_root);
assert.equal(proof.replay_root, proof.state_root);
console.log('Local replay fields: PASS');
JS
```

Also inspect `sessions/session-0001.json`, `gameplay/arena-state.json`, and `journals/journal.jsonl` under the runtime directory. These are producer observations, not independent acceptance or a new receipt policy.

## Bounded executable check

On the quickstart implementation branch, run:

```bash
python3 scripts/quickstart/smoke.py
python3 -m unittest discover -s tests/quickstart -v
```

The helper defaults to the full path, enforces the exact tool pins and preinstalled Wasm target, restores existing offline vendor, then executes the real CLI and checks package bytes/digest and replay fields. It retains a fresh external temporary project/runtime directory, per-command logs, exits, `result.json` and `SHA256SUMS`. Cargo compilation has bounded timeouts; native target output is disposable. Hashes cover logs and generated evidence, excluding the disposable native Cargo target cache. It refuses an output parent inside the checkout and any existing upstream guest scratch directory. It does not run release/deploy/signing commands or broad contributor gates.

For a host without Rust or the pinned Node, use the explicitly limited source check:

```bash
python3 scripts/quickstart/smoke.py --source-only
```

It records Node pin mismatch and says package/native/replay `NOT_RUN`. A source-only pass cannot replace the default native check or contributor gates.

## Troubleshooting and gate boundary

| Symptom | Next step |
| --- | --- |
| Node/Rust mismatch or missing cargo/rustup | Install the exact pinned tools; report native checks `NOT_RUN` until available. |
| Missing Wasm target | Install the 1.95.0 target before offline packaging. |
| Missing crate or vendor checksum failure | Run the existing checksum-verified restore; obtain the exact committed bundle if missing. Do not edit hashes/locks. |
| Guest scratch already exists | Preserve it; use a separate disposable environment. |
| Package/play-local exits nonzero | Read the retained step log and exact error; do not infer native success from Build/Test. |
| Replay missing or roots differ | Treat the local path as failed; retain evidence for review. |

The canonical Contributor Gate remains the three commands in `CONTRIBUTING.md`: prerequisites, developer onboarding, and reference-world verification. The World Artifact Gate is separate and produces/certifies the canonical `world.evr`; this quickstart does not replace it. Broad gates, restricted historical qualification, deployment and release are outside this bounded helper.

Source references at the pin: [CLI](../../creator-sdk/cli/everarcade.mjs), [arena template](../../creator-sdk/templates/arena), [onboarding script](../../scripts/validate_developer_onboarding.sh), [CI](../../.github/workflows/onboarding.yml), and [runtime package loader](../../runtime/everarcade-runtime/src/runtime/package_loader.rs).
