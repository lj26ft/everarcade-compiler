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
REPORT="deployment/reports/operator_control_plane_readiness.md"
mkdir -p deployment/reports
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test -p execution-core --test operator_control_plane_tests "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'EOF'
# operator_control_plane_readiness.md

Classification: Ready

Validation command completed successfully with CARGO_BUILD_JOBS=1 by default.
EOF
echo "report=$REPORT"
