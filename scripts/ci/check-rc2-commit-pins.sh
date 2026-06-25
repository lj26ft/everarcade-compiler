#!/usr/bin/env bash
set -euo pipefail

EXPECTED_COMMIT="fe51c1ce5be6df888dfaae203d5632580a045f2e"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

mapfile -t pins < <(
  rg --no-heading --line-number --glob '!target/**' --glob '!node_modules/**' --glob '!vendor/**' \
    '(commit|checkout|pin(ned)?|gate target).*([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.)|([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.).*(commit|checkout|pin(ned)?|gate target)' \
    TRUST_ROOT.md \
    OPEN_SOURCE_CANDIDATE_RC2.md \
    GATE_PROOFS.md \
    PAYLOAD_BINDING.md \
    REVIEW_TRUST_CHAIN.md \
    OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE \
    2>/dev/null \
  | rg '[[:xdigit:]]{40}' || true
)

conflicts=()
for pin in "${pins[@]}"; do
  if [[ "$pin" != *"$EXPECTED_COMMIT"* ]]; then
    conflicts+=("$pin")
  fi
done

if (( ${#conflicts[@]} > 0 )); then
  printf 'ERROR: Found stale or conflicting RC2 commit pin. Expected %s only.\n' "$EXPECTED_COMMIT" >&2
  printf '%s\n' "${conflicts[@]}" >&2
  exit 1
fi

printf 'RC2 commit pins are consistent: %s\n' "$EXPECTED_COMMIT"
