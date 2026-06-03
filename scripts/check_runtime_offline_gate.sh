#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/runtime_offline_gate_report.txt"

if [[ ! -d "$ROOT_DIR/vendor" ]]; then
  echo "vendor/ is missing. Run: bash scripts/restore_vendor_artifact.sh" >&2
  exit 1
fi

mkdir -p "$REPORT_DIR"
: > "$REPORT_PATH"

run_step() {
  local description="$1"
  shift
  local tmp_out
  tmp_out="$(mktemp)"

  {
    printf '== %s ==\n' "$description"
    printf '$'
    printf ' %q' "$@"
    printf '\n'
  } >> "$REPORT_PATH"

  "$@" >"$tmp_out" 2>&1
  local status=$?

  if [[ "$status" -eq 0 ]]; then
    printf 'status: pass\n\n' >> "$REPORT_PATH"
    printf '%s passes\n' "$*"
  else
    {
      printf 'status: fail (%s)\n' "$status"
      printf -- '-- output tail --\n'
      tail -n 80 "$tmp_out"
      printf '\n'
    } >> "$REPORT_PATH"
    cat "$tmp_out" >&2
    rm -f "$tmp_out"
    return "$status"
  fi

  rm -f "$tmp_out"
  return 0
}

status=0

run_step "Cargo metadata offline locked" cargo metadata --offline --locked --format-version 1 || status=$?
run_step "Runtime cargo check offline locked" cargo check -p everarcade-runtime --offline --locked || status=$?
run_step "Runtime cargo tests offline locked" cargo test -p everarcade-runtime --tests --offline --locked || status=$?

printf 'Runtime offline gate report written to %s\n' "$REPORT_PATH"
exit "$status"
