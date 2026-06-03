#!/usr/bin/env bash
set -uo pipefail

REPORT_DIR="reports"
REPORT_PATH="${REPORT_DIR}/runtime_offline_gate_diagnostic.txt"

mkdir -p "${REPORT_DIR}"
: > "${REPORT_PATH}"

run_step() {
  local description="$1"
  shift

  {
    printf '\n== %s ==\n' "${description}"
    printf '$'
    printf ' %q' "$@"
    printf '\n'
  } >> "${REPORT_PATH}"

  "$@" >> "${REPORT_PATH}" 2>&1
  local status=$?

  {
    printf '\n[exit status: %s]\n' "${status}"
  } >> "${REPORT_PATH}"

  return "${status}"
}

status=0

run_step "Cargo metadata offline locked" cargo metadata --offline --locked || status=$?
run_step "Runtime cargo check offline locked" cargo check -p everarcade-runtime --offline --locked || status=$?
run_step "Runtime cargo tests offline locked" cargo test -p everarcade-runtime --tests --offline --locked || status=$?

printf 'Runtime offline gate diagnostic written to %s\n' "${REPORT_PATH}"
exit "${status}"
