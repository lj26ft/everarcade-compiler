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