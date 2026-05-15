#!/usr/bin/env bash
set -euo pipefail

# Policy:
# - Canonical deterministic vector fixtures are allowed as tracked assets.
# - Runtime-generated receipts/checkpoints/state artifacts must remain untracked.

tracked_files="$(git ls-files)"

# Explicit runtime artifact path patterns that must never be tracked.
RUNTIME_PATTERNS=(
  '(^|/)\.everarcade($|/)'
  '(^|/)target($|/)'
  '(^|/)tmp($|/)'
  '(^|/)temp($|/)'
  '^state($|/)'
  '^\.everarcade($|/)'
  '\.tmp$'
  '\.log$'
)

violations=""
for pattern in "${RUNTIME_PATTERNS[@]}"; do
  matches="$(printf '%s\n' "$tracked_files" | grep -E "$pattern" || true)"
  if [[ -n "$matches" ]]; then
    violations+="$matches"$'\n'
  fi
done

if [[ -n "$violations" ]]; then
  echo 'Tracked runtime-generated artifacts detected. Remove from Git tracking before merge.'
  printf '%s' "$violations" | sed '/^$/d' | sort -u
  exit 1
fi

echo 'No tracked runtime-generated artifacts detected. Canonical vector fixtures are allowed.'
