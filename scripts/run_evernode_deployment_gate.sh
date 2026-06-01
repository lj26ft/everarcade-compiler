#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/evernode_deployment_gate_run.md"
test -f deployment/evernode/runtime/multinode-federation-load-manifest.toml
bash scripts/generate_evernode_packages.sh >/dev/null
mkdir -p "$(dirname "$REPORT")"
for name in runtime world deployment; do
  test -f "deployment/evernode/runtime/arena-vanguard-${name}.tar.gz"
  test -f "deployment/evernode/runtime/arena-vanguard-${name}.tar.gz.sha256"
  test -f "deployment/evernode/runtime/arena-vanguard-${name}.tar.gz.sig"
  test -f "deployment/evernode/runtime/arena-vanguard-${name}.receipt.json"
  (cd deployment/evernode/runtime && sha256sum -c "arena-vanguard-${name}.tar.gz.sha256") >/dev/null
done
cargo test -p execution-core --test evernode_deployment_tests "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT_EOF'
# EverNode Deployment Gate Run

status: passed
package verification: passed
deployment verification: passed
replay continuity: passed
checkpoint continuity: passed
multi-node federation load: passed
REPORT_EOF
echo "evernode deployment gate passed; report=$REPORT"

bash scripts/build_evernode_packages.sh
(cd deployment/evernode/runtime && sha256sum -c packages.sha256 >/dev/null)
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test evernode_deployment_tests "$@"

mkdir -p deployment/reports
cat > deployment/reports/evernode_launch_gate.md <<'RPT'
# EverNode Launch Gate

| Domain | Classification | Evidence |
| --- | --- | --- |
| generated package/runtime gate | Ready | Runtime, world, and deployment packages are generated with deterministic manifests, hashes, signatures, receipts, and package validation. |
| live EverNode deployment | Partially Ready | Deployment manifests, recovery checks, and local/loopback federation gates are validated; no live EverNode production deployment is performed by this repository. |
| unsupported live operations | Not Ready | Operations outside the documented deploy/start/stop/restart/recover/verify surface remain rejected and are not certified for live production use. |
| operator tooling | Partially Ready | Operator guides and validation scripts exist; production service manager integration and host operations remain operator-owned. |
| load validation | Ready | Four-node loopback TCP federation load gate validates balanced deterministic message load within capacity. |

## Gate Boundaries
- Ready status is limited to the validated generated package/runtime gate and deterministic validation artifacts.
- Live EverNode deployment remains Partially Ready until an actual production deployment is performed and evidenced.
- Unsupported live operations remain Not Ready and must not be treated as production-ready behavior.
- Generated `.tar.gz` and `.sig` files are not source controlled; rebuild with `scripts/build_evernode_packages.sh` and validate `deployment/evernode/runtime/packages.sha256`.
RPT
cp deployment/reports/evernode_launch_gate.md deployment/reports/evernode_deployment_gate.md

echo "evernode deployment gate complete"
