#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PROJECT="$(mktemp -d)"
trap 'rm -rf "$PROJECT"' EXIT

cp examples/world-factory/frontier-settlement/world-blueprint.json "$PROJECT/"
cp examples/world-factory/frontier-settlement/world-contract-plan.json "$PROJECT/"
mkdir -p "$PROJECT/out"

CLI=(node creator-sdk/cli/everarcade.mjs)
export EVERARCADE_DETERMINISTIC_ATTEST=1

"${CLI[@]}" world factory generate --project "$PROJECT"
"${CLI[@]}" world factory verify --project "$PROJECT"
"${CLI[@]}" world factory boot --project "$PROJECT"
"${CLI[@]}" world factory run --project "$PROJECT" --ticks 100
"${CLI[@]}" world factory replay --project "$PROJECT"
"${CLI[@]}" world factory deploy --project "$PROJECT"
"${CLI[@]}" world attest create --project "$PROJECT"

TRUSTED_PUBLIC_KEY="$(tr -d '\n' < fixtures/trust-root/test-attester-public-key.txt)"
if "${CLI[@]}" world attest verify --project "$PROJECT" --trusted-public-key "$TRUSTED_PUBLIC_KEY"; then
  printf 'RC2 self-attested-fork fixture: FAIL - bad self-attested package passed verification\n' >&2
  exit 1
fi

printf 'RC2 self-attested-fork fixture: PASS - bad self-attested package failed verification\n'
