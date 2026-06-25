#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_EXPORT_COMMIT="fe51c1ce5be6df888dfaae203d5632580a045f2e"
REVIEWER_ENTRY_COMMIT="53a17567e826c5d4f9b083e490cf1568bfe7534e"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

mapfile -t pins < <(
  rg --no-heading --line-number --glob '!target/**' --glob '!node_modules/**' --glob '!vendor/**' \
    '(artifact/export commit|reviewer-entry commit|commit|checkout|pin(ned)?|gate target).*([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.)|([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.).*(artifact/export commit|reviewer-entry commit|commit|checkout|pin(ned)?|gate target)' \
    README.md \
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
  if [[ "$pin" == *"$ARTIFACT_EXPORT_COMMIT"* ]]; then
    if [[ "$pin" != *"artifact/export commit"* ]]; then
      conflicts+=("$pin")
    fi
  elif [[ "$pin" == *"$REVIEWER_ENTRY_COMMIT"* ]]; then
    if [[ "$pin" != *"reviewer-entry commit"* && "$pin" != *"git checkout $REVIEWER_ENTRY_COMMIT"* ]]; then
      conflicts+=("$pin")
    fi
  else
    conflicts+=("$pin")
  fi
done

if (( ${#conflicts[@]} > 0 )); then
  printf 'ERROR: Found stale or conflicting RC2 commit pin. Expected artifact/export %s only when labeled, and reviewer-entry %s for checkout/reviewer commands.\n' "$ARTIFACT_EXPORT_COMMIT" "$REVIEWER_ENTRY_COMMIT" >&2
  printf '%s\n' "${conflicts[@]}" >&2
  exit 1
fi

printf 'RC2 commit pins are consistent: artifact/export %s; reviewer-entry %s\n' "$ARTIFACT_EXPORT_COMMIT" "$REVIEWER_ENTRY_COMMIT"
