#!/usr/bin/env bash
set -euo pipefail

# Policy:
# - Canonical deterministic vector fixtures are allowed as tracked assets.
# - Runtime-generated tarballs, signatures, receipts, dist outputs, and transient
#   runtime state must remain untracked.

tracked_files="$(git ls-files)"

RUNTIME_PATTERNS=(
  '(^|/)\.everarcade($|/)'
  '(^|/)target($|/)'
  '(^|/)tmp($|/)'
  '(^|/)temp($|/)'
  '^state($|/)'
  '^\.everarcade($|/)'
  '\.tmp$'
  '^dist($|/)'
  '\.tar\.gz$'
  '\.sig$'
  '\.pkg$'
  '^validation_logs($|/)'
  '^validation_logs\.tar\.gz$'
  '(^|/)receipt\.json$'
)

allowed_fixture_patterns=(
  '^deployment/evernode/runtime/package-input/.*/receipt\.json$'
)

is_allowed_fixture() {
  local file="$1"
  local pattern
  for pattern in "${allowed_fixture_patterns[@]}"; do
    if [[ "$file" =~ $pattern ]]; then
      return 0
    fi
  done
  return 1
}

violations=""
for pattern in "${RUNTIME_PATTERNS[@]}"; do
  while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    if ! is_allowed_fixture "$file"; then
      violations+="$file"$'\n'
    fi
  done < <(printf '%s\n' "$tracked_files" | grep -E "$pattern" || true)
done

if [[ -n "$violations" ]]; then
  echo 'Tracked runtime-generated artifacts detected. Remove from Git tracking before merge.'
  printf '%s' "$violations" | sed '/^$/d' | sort -u
  exit 1
fi

echo 'No tracked runtime-generated artifacts detected. Canonical vector fixtures are allowed.'
