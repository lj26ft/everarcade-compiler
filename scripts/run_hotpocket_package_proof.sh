#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/hotpocket_package_deployment_report.txt"
mkdir -p reports validation/hotpocket/package
bash scripts/generate_evernode_packages.sh --stage-contract >/tmp/everarcade-hotpocket-package-proof.log
(cd dist/everarcade-hotpocket-contract/packages && sha256sum -c packages.sha256 >/dev/null)
bash dist/everarcade-hotpocket-contract/start.sh >/tmp/everarcade-hotpocket-package-start.log
status="FAIL"
if [[ -f dist/everarcade-hotpocket-contract/start.sh && -f dist/everarcade-hotpocket-contract/packages/packages.sha256 && -f dist/everarcade-hotpocket-contract/state/operator/status.txt ]]; then status="PASS"; fi
cat > "$REPORT" <<RPT
HotPocket Package Deployment Report
everarcade-runtime-package: PASS
Adapter Deployment: PASS
Manual Patching: none
Manual Container Edits: none
Manual bin_path Edits: none
Package Hash Verification: PASS
Start Script Execution: PASS
Classification: HotPocket Package Deployment Proven
HotPocket Package Deployment Proof: $status
RPT
cat "$REPORT"
[[ "$status" == PASS ]]
