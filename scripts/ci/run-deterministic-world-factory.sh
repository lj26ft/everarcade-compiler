#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=../lib/common.sh
source "$ROOT/scripts/lib/common.sh"

cd "$ROOT"

PROJECT="${WORLD_FACTORY_PROJECT:-examples/world-factory/frontier-settlement}"
CLI=(node creator-sdk/cli/everarcade.mjs)
RELEASE_ARCHIVE="${RELEASE_ARCHIVE:-dist/everarcade-world-factory-release.tar.gz}"

pass() { printf '%s: PASS\n' "$1"; }
fail() { printf '%s: FAIL - %s\n' "$1" "$2" >&2; exit 1; }

section() { printf '\n== %s ==\n' "$1"; }

FIXTURE_DIR="$ROOT/examples/world-factory/frontier-settlement/fixtures"

copy_factory_seed() {
  local dest="$1"
  rm -rf "$dest"
  mkdir -p "$dest/out"
  cp "$ROOT/examples/world-factory/frontier-settlement/world-blueprint.json" "$dest/"
  cp "$ROOT/examples/world-factory/frontier-settlement/world-contract-plan.json" "$dest/"
  cp "$FIXTURE_DIR/attester-ed25519-private.pem" "$dest/out/attester-ed25519-private.pem"
}

export EVERARCADE_DETERMINISTIC_ATTEST=1

run_factory_pipeline() {
  local project="$1"
  local ticks="${2:-100}"
  "${CLI[@]}" world factory generate --project "$project"
  "${CLI[@]}" world factory verify --project "$project"
  "${CLI[@]}" world factory boot --project "$project"
  "${CLI[@]}" world factory run --project "$project" --ticks "$ticks"
  "${CLI[@]}" world factory replay --project "$project"
  "${CLI[@]}" world factory deploy --project "$project"
  "${CLI[@]}" world attest create --project "$project"
  local key
  key="$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' "$ROOT/TRUST_ROOT.md")"
  "${CLI[@]}" world attest verify --project "$project" --trusted-public-key "$key"
  node "$ROOT/specs/world-evr-package/verify-package-v1.mjs" "$project/out/world.evr"
}

section "Prerequisites and vendor"
bash "$ROOT/scripts/ensure_vendor_offline.sh"
bash "$ROOT/scripts/check_prerequisites.sh"

section "Toolchain gates"
cargo fmt --all --check
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build -p everarcade-cli --offline --locked
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build -p everarcade-cli --release --offline --locked
CARGO_NET_OFFLINE=true cargo test -p everarcade-cli --offline --locked --quiet -- --test-threads=1
node --check creator-sdk/cli/everarcade.mjs

section "World Factory canonical pipeline"
rm -rf "$PROJECT/out"
mkdir -p "$PROJECT/out"
cp "$FIXTURE_DIR/attester-ed25519-private.pem" "$PROJECT/out/attester-ed25519-private.pem"
run_factory_pipeline "$PROJECT" 100

section "Determinism check (two isolated generates)"
RUN_A="$(mktemp -d)"
RUN_B="$(mktemp -d)"
trap 'rm -rf "$RUN_A" "$RUN_B"' EXIT
copy_factory_seed "$RUN_A"
copy_factory_seed "$RUN_B"
"${CLI[@]}" world factory generate --project "$RUN_A"
"${CLI[@]}" world factory generate --project "$RUN_B"
diff -u "$RUN_A/out/world.evr/expected-package-hash.txt" "$RUN_B/out/world.evr/expected-package-hash.txt"
diff -u "$RUN_A/out/world-factory-report.json" "$RUN_B/out/world-factory-report.json"
run_factory_pipeline "$RUN_A" 100
run_factory_pipeline "$RUN_B" 100
diff -u "$RUN_A/out/deploy/deployment-manifest.json" "$RUN_B/out/deploy/deployment-manifest.json"
diff -u "$RUN_A/out/runtime/world-factory-runtime-report.json" "$RUN_B/out/runtime/world-factory-runtime-report.json"
pass "determinism"

section "Release bundle"
"${CLI[@]}" release build --project "$PROJECT"
"${CLI[@]}" release inspect "$RELEASE_ARCHIVE"
"${CLI[@]}" release smoke-test "$RELEASE_ARCHIVE"
release_bytes="$(wc -c < "$RELEASE_ARCHIVE" | tr -d ' ')"
if [[ "$release_bytes" -gt $((100 * 1024 * 1024)) ]]; then
  fail "release size gate" "bundle exceeds 100 MB ($release_bytes bytes)"
fi
pass "release bundle"

section "Complete"
printf 'Deterministic World Factory CI: PASS\n'