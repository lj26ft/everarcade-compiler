#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_EXPORT_COMMIT="fe51c1ce5be6df888dfaae203d5632580a045f2e"
REVIEWER_ENTRY_COMMIT="53a17567e826c5d4f9b083e490cf1568bfe7534e"
STALE_RC2_COMMIT_PREFIX="b6d553d"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

INSPECT_PATHS=(
  README.md
  TRUST_ROOT.md
  OPEN_SOURCE_CANDIDATE_RC2.md
  GATE_PROOFS.md
  PAYLOAD_BINDING.md
  REVIEW_TRUST_CHAIN.md
  OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE
)

if command -v rg >/dev/null 2>&1; then
  SEARCH_TOOL="rg"
else
  SEARCH_TOOL="grep"
fi

search_ran=0
inspected_files=0

for path in "${INSPECT_PATHS[@]}"; do
  if [[ ! -e "$path" ]]; then
    printf 'ERROR: Expected RC2 commit-pin inspection path is missing: %s\n' "$path" >&2
    exit 1
  fi

  if [[ -d "$path" ]]; then
    count=$(find "$path" -type f | wc -l | tr -d '[:space:]')
    inspected_files=$((inspected_files + count))
  elif [[ -f "$path" ]]; then
    inspected_files=$((inspected_files + 1))
  else
    printf 'ERROR: Expected RC2 commit-pin inspection path is not a file or directory: %s\n' "$path" >&2
    exit 1
  fi
done

if (( inspected_files == 0 )); then
  printf 'ERROR: RC2 commit-pin checker inspected zero files.\n' >&2
  exit 1
fi

search() {
  search_ran=1
  if [[ "$SEARCH_TOOL" == "rg" ]]; then
    rg --no-heading --line-number --glob '!target/**' --glob '!node_modules/**' --glob '!vendor/**' "$1" "${INSPECT_PATHS[@]}"
  else
    grep -RIn --exclude-dir=target --exclude-dir=node_modules --exclude-dir=vendor -E "$1" "${INSPECT_PATHS[@]}"
  fi
}

require_match() {
  description="$1"
  pattern="$2"
  tmp="$(mktemp)"
  status=0
  search "$pattern" >"$tmp" || status=$?
  if (( status == 0 )); then
    rm -f "$tmp"
    return 0
  fi
  rm -f "$tmp"
  if (( status == 1 )); then
    printf 'ERROR: Missing required RC2 commit-pin validation: %s\n' "$description" >&2
  else
    printf 'ERROR: Search failed while checking required RC2 commit-pin validation (%s) with exit code %s.\n' "$description" "$status" >&2
  fi
  exit 1
}

forbid_match() {
  description="$1"
  pattern="$2"
  tmp="$(mktemp)"
  status=0
  search "$pattern" >"$tmp" || status=$?
  if (( status == 0 )); then
    printf 'ERROR: Forbidden RC2 commit-pin text found: %s\n' "$description" >&2
    cat "$tmp" >&2
    rm -f "$tmp"
    exit 1
  fi
  rm -f "$tmp"
  if (( status == 1 )); then
    return 0
  fi
  printf 'ERROR: Search failed while checking forbidden RC2 commit-pin text (%s) with exit code %s.\n' "$description" "$status" >&2
  exit 1
}

# Required labeled pins and reviewer checkout instructions.
require_match "artifact/export commit $ARTIFACT_EXPORT_COMMIT" "artifact/export commit.*$ARTIFACT_EXPORT_COMMIT|$ARTIFACT_EXPORT_COMMIT.*artifact/export commit"
require_match "reviewer-entry commit $REVIEWER_ENTRY_COMMIT" "reviewer-entry commit.*$REVIEWER_ENTRY_COMMIT|$REVIEWER_ENTRY_COMMIT.*reviewer-entry commit"
require_match "reviewer checkout $REVIEWER_ENTRY_COMMIT" "git checkout[[:space:]]+$REVIEWER_ENTRY_COMMIT"

# Known stale pin and ambiguous reviewer-path wording must never reappear.
forbid_match "stale RC2 commit prefix $STALE_RC2_COMMIT_PREFIX" "$STALE_RC2_COMMIT_PREFIX"
forbid_match "ambiguous same commit wording" "same commit"

pin_matches="$(mktemp)"
pin_pattern='(artifact/export commit|reviewer-entry commit|commit|checkout|pin(ned)?|gate target).*([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.)|([[:xdigit:]]{40}|[[:xdigit:]]{7}\.\.\.).*(artifact/export commit|reviewer-entry commit|commit|checkout|pin(ned)?|gate target)'
pin_status=0
search "$pin_pattern" >"$pin_matches" || pin_status=$?
if (( pin_status == 1 )); then
  printf 'ERROR: No RC2 commit-pin candidates were found; validation did not run against real pins.\n' >&2
  rm -f "$pin_matches"
  exit 1
elif (( pin_status != 0 )); then
  printf 'ERROR: Search failed while collecting RC2 commit-pin candidates with exit code %s.\n' "$pin_status" >&2
  rm -f "$pin_matches"
  exit 1
fi

conflicts="$(mktemp)"
while IFS= read -r pin; do
  case "$pin" in
    *"$ARTIFACT_EXPORT_COMMIT"*)
      case "$pin" in
        *"artifact/export commit"*) ;;
        *) printf '%s\n' "$pin" >>"$conflicts" ;;
      esac
      ;;
    *"$REVIEWER_ENTRY_COMMIT"*)
      case "$pin" in
        *"reviewer-entry commit"*|*"git checkout $REVIEWER_ENTRY_COMMIT"*) ;;
        *) printf '%s\n' "$pin" >>"$conflicts" ;;
      esac
      ;;
    *)
      printf '%s\n' "$pin" >>"$conflicts"
      ;;
  esac
done <"$pin_matches"

if [[ -s "$conflicts" ]]; then
  printf 'ERROR: Found stale or conflicting RC2 commit pin. Expected artifact/export %s only when labeled, and reviewer-entry %s for checkout/reviewer commands.\n' "$ARTIFACT_EXPORT_COMMIT" "$REVIEWER_ENTRY_COMMIT" >&2
  cat "$conflicts" >&2
  rm -f "$pin_matches" "$conflicts"
  exit 1
fi

rm -f "$pin_matches" "$conflicts"

if (( search_ran == 0 )); then
  printf 'ERROR: RC2 commit-pin checker completed without running a search.\n' >&2
  exit 1
fi

printf 'PASS: RC2 commit pins validated across %s files with %s: artifact/export %s; reviewer-entry %s\n' "$inspected_files" "$SEARCH_TOOL" "$ARTIFACT_EXPORT_COMMIT" "$REVIEWER_ENTRY_COMMIT"
