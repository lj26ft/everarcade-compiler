#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    *) echo "unsupported argument: $arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/arena_vanguard_live_deployment_report.md"
mkdir -p deployment/reports
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test operator_control_plane_tests test_arena_vanguard_live_deployment "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'EOF'
# arena_vanguard_live_deployment_report.md

Classification: Ready

Validation command completed successfully with CARGO_BUILD_JOBS=1 by default.
EOF
echo "report=$REPORT"
