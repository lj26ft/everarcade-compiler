#!/usr/bin/env bash
# Shared helpers for EverArcade validation scripts.

text_matches() {
  local pattern="$1"
  local file="$2"
  if command -v rg >/dev/null 2>&1; then
    rg -q "$pattern" "$file"
  else
    grep -qE "$pattern" "$file"
  fi
}

text_matches_stream() {
  local pattern="$1"
  if command -v rg >/dev/null 2>&1; then
    rg -q "$pattern"
  else
    grep -qE "$pattern"
  fi
}

vendor_metadata_offline_ok() {
  local root="${1:-.}"
  (
    cd "$root"
    CARGO_NET_OFFLINE=true cargo metadata --offline --locked --format-version 1 >/dev/null 2>&1
  )
}

vendor_tree_present() {
  local root="${1:-.}"
  [[ -d "$root/vendor" ]] && [[ -n "$(find "$root/vendor" -maxdepth 1 -type d -name 'bincode*' -print -quit 2>/dev/null)" ]]
}

vendor_offline_ok() {
  local root="${1:-.}"
  vendor_tree_present "$root" && vendor_metadata_offline_ok "$root"
}

print_vendor_fix_hint() {
  cat >&2 <<'HINT'
Offline Cargo failed. Common fixes:
  1. bash scripts/ensure_vendor_offline.sh
  2. Maintainer regen: bash scripts/vendor_deps.sh  (network required once)
  3. Commit updated dist/vendor.tar.gz + dist/vendor.tar.gz.sha256
Creator SDK play-local must run cargo from the repo root (not /tmp) so vendor/ applies.
HINT
}

offline_cargo_check_workspace() {
  local root="${1:-.}"
  local log="${2:-/tmp/everarcade-offline-check.log}"
  (
    cd "$root"
    CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" \
      cargo check --offline --locked >"$log" 2>&1
  )
}