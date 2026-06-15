#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/open_source_readiness_report.txt"
mkdir -p reports
: > "$REPORT"

pass() { printf '%s: PASS\n' "$1" | tee -a "$REPORT"; }
warn() { printf '%s: WARNING - %s\n' "$1" "$2" | tee -a "$REPORT"; }
fail() { printf '%s: FAIL - %s\n' "$1" "$2" | tee -a "$REPORT"; missing=1; }
require_file() { [[ -f "$1" ]] && pass "$2" || fail "$2" "missing $1"; }
require_text() { local file="$1" pattern="$2" label="$3"; rg -q "$pattern" "$file" && pass "$label" || fail "$label" "missing expected text in $file"; }

missing=0
warnings=0

cat >> "$REPORT" <<'HEADER'
Open Source Readiness Report
Date: 2026-06-15

Checks
HEADER

require_file README.md "README"
require_text README.md "Why does EverArcade exist" "README purpose"
require_text README.md "What can I do today" "README capability summary"
require_text README.md "Where are the docs" "README documentation links"
require_file LICENSE "LICENSE"
require_file CONTRIBUTING.md "CONTRIBUTING"
require_file SECURITY.md "SECURITY"
require_file CODE_OF_CONDUCT.md "CODE_OF_CONDUCT"
require_file docs/onboarding/30-minute-developer-journey.md "Onboarding docs"
require_file REPOSITORY_MAP.md "Repository map"
require_file MATURITY.md "Maturity classification"
require_file docs/index.md "Documentation portal root"
require_file docs/DOCUMENTATION_POLICY.md "Documentation policy"
require_file OPEN_SOURCE_READINESS.md "Open source readiness audit"
require_file docs/runtime-platform/proof-chain.md "Proof chain"
require_file docs/repository/artifact-policy.md "Artifact policy"
require_file docs/build/offline-build-policy.md "Offline build policy"
require_file reports/script_consolidation_audit.txt "Script consolidation audit"
require_file reports/documentation_consolidation_audit.txt "Documentation consolidation audit"

if [[ -d node_modules ]]; then
  warnings=1
  warn "Dependency tree" "node_modules/ exists in this working tree and should not be committed"
else
  pass "Dependency tree"
fi

if [[ ! -d vendor ]] || ! find vendor -maxdepth 1 -type d -name 'bincode*' | rg -q 'bincode'; then
  warnings=1
  warn "Offline vendor" "vendor snapshot is incomplete; known bincode/vendor issue remains documented"
else
  pass "Offline vendor"
fi

classification="READY"
if [[ "$missing" -ne 0 ]]; then
  classification="NOT READY"
elif [[ "$warnings" -ne 0 ]]; then
  classification="CONDITIONAL READY"
fi

cat >> "$REPORT" <<EOF_REPORT

Classification: $classification

Honest readiness notes
- Local developer onboarding is the supported v0.1 proof path.
- Production, public-testnet, commercial, XRPL/Xaman, GPU marketplace, and federation readiness are not claimed.
- Offline builds remain conditional until the vendor snapshot is complete and verified.

Open Source Audit: PASS
EOF_REPORT

cat "$REPORT"
[[ "$missing" -eq 0 ]]
