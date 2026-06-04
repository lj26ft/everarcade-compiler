#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-evernode-$VERSION"
DIST_DIR="$ROOT/dist"
REPORT_DIR="$ROOT/reports"
PACKAGE="$DIST_DIR/$NAME.tar.gz"
CHECKSUM="$PACKAGE.sha256"
STAGING_PARENT="$ROOT/tmp/evernode-package-build"
STAGING="$STAGING_PARENT/$NAME"
REPORT="$REPORT_DIR/evernode_package_build_report.txt"

mkdir -p "$DIST_DIR" "$REPORT_DIR" "$STAGING_PARENT"
rm -rf "$STAGING" "$PACKAGE" "$CHECKSUM"
mkdir -p "$STAGING"

# Keep generated package artifacts out of commits even though they live under dist/.
for ignore_pattern in \
  '/dist/everarcade-evernode-v0.1.tar.gz' \
  '/dist/everarcade-evernode-v0.1.tar.gz.sha256'; do
  grep -qxF "$ignore_pattern" "$ROOT/.git/info/exclude" || printf '%s\n' "$ignore_pattern" >> "$ROOT/.git/info/exclude"
done

required_paths=(
  evernode/lease/deployment-manifest.txt
  runtime/config
  runtime/deployment
  node
  hotpocket/adapter
  docs/runtime-platform/evernode-deployment-v0.1.md
  scripts/evernode_bootstrap.sh
  scripts/validate_evernode_deployment.sh
  scripts/certify_evernode_deployment.sh
  scripts/node_init.sh
  scripts/node_start.sh
  scripts/node_checkpoint.sh
  scripts/node_replay.sh
  scripts/node_restore.sh
  scripts/node_stop.sh
  scripts/node_doctor.sh
  scripts/validate_hotpocket_integration.sh
  scripts/certify_hotpocket_integration.sh
)

status_for_paths() {
  local status="PASS" path
  for path in "$@"; do
    if [[ ! -e "$path" ]]; then
      status="FAIL"
      break
    fi
  done
  printf '%s' "$status"
}

assets_status="$(status_for_paths "${required_paths[@]}")"
if [[ "$assets_status" != PASS ]]; then
  cat > "$REPORT" <<REPORT_BODY
Evernode Package Build Report
Required Assets: $assets_status
Package: $PACKAGE
Checksum: $CHECKSUM
Package Build: FAIL
REPORT_BODY
  echo "Package Build: FAIL"
  exit 1
fi

mkdir -p "$STAGING/evernode" "$STAGING/runtime" "$STAGING/node" "$STAGING/hotpocket" "$STAGING/docs/runtime-platform" "$STAGING/scripts" "$STAGING/reports"
cp -R evernode/lease evernode/runtime evernode/node evernode/hotpocket evernode/checkpoints evernode/journals evernode/backups evernode/logs evernode/reports "$STAGING/evernode/"
cp -R runtime/config runtime/deployment "$STAGING/runtime/"
cp -R node/config node/state node/worlds node/journals node/checkpoints node/backups node/reports node/logs "$STAGING/node/"
cp -R hotpocket/adapter hotpocket/input hotpocket/output hotpocket/checkpoint hotpocket/replay hotpocket/settlement hotpocket/status "$STAGING/hotpocket/"
cp docs/runtime-platform/evernode-deployment-v0.1.md "$STAGING/docs/runtime-platform/"
cp \
  scripts/evernode_bootstrap.sh \
  scripts/validate_evernode_deployment.sh \
  scripts/certify_evernode_deployment.sh \
  scripts/node_init.sh \
  scripts/node_start.sh \
  scripts/node_checkpoint.sh \
  scripts/node_replay.sh \
  scripts/node_restore.sh \
  scripts/node_stop.sh \
  scripts/node_doctor.sh \
  scripts/node_common.sh \
  scripts/validate_hotpocket_integration.sh \
  scripts/certify_hotpocket_integration.sh \
  scripts/runtime_doctor.sh \
  scripts/runtime_start.sh \
  scripts/runtime_stop.sh \
  scripts/runtime_status.sh \
  "$STAGING/scripts/"

cat > "$STAGING/PACKAGE_MANIFEST.txt" <<MANIFEST
EverArcade Evernode Deployment Package v0.1
Package: $NAME
Evernode Layout: evernode/
Runtime Appliance Assets: runtime/config, runtime/deployment, scripts/runtime_*.sh
Protocol Node Assets: node/, scripts/node_*.sh
HotPocket Assets: hotpocket/, scripts/validate_hotpocket_integration.sh, scripts/certify_hotpocket_integration.sh
Documentation: docs/runtime-platform/evernode-deployment-v0.1.md
Live Evernode Registration: disabled
Public Lease Deployment: disabled
Networking: disabled
Consensus: disabled
XRPL RPC: disabled
Hooks: disabled
Xaman Signing: disabled
MANIFEST

# Remove generated runtime payload noise from the staged package while preserving
# source assets, documentation, operation scripts, manifests, and layout markers.
rm -rf "$STAGING/evernode/install"
find "$STAGING/evernode/backups" -mindepth 1 ! -name .gitkeep -exec rm -rf {} + 2>/dev/null || true
find "$STAGING/evernode/checkpoints" -type f ! -name .gitkeep -delete
find "$STAGING/evernode/journals" -type f ! -name .gitkeep -delete
find "$STAGING/evernode/logs" -type f ! -name .gitkeep -delete
find "$STAGING/evernode/reports" -type f ! -name .gitkeep -delete
find "$STAGING/node" -type f ! -name .gitkeep -delete
find "$STAGING/node/backups" -mindepth 1 ! -name .gitkeep -exec rm -rf {} + 2>/dev/null || true
find "$STAGING/node/worlds" -mindepth 1 ! -name .gitkeep -exec rm -rf {} + 2>/dev/null || true
find "$STAGING/node" -type d -empty -delete
find "$STAGING/hotpocket" \( -name '*.env' -o -name '*_root' -o -name 'layout.txt' \) -delete
find "$STAGING" -type f \( -name '*.tmp' -o -name '*.log' \) -delete

tar -C "$STAGING_PARENT" -czf "$PACKAGE" "$NAME"
sha256sum "$PACKAGE" > "$CHECKSUM"

package_status="FAIL"
checksum_status="FAIL"
manifest_status="FAIL"
[[ -s "$PACKAGE" ]] && package_status="PASS"
[[ -s "$CHECKSUM" ]] && checksum_status="PASS"
[[ -s "$STAGING/PACKAGE_MANIFEST.txt" ]] && manifest_status="PASS"
overall="FAIL"
if [[ "$package_status" == PASS && "$checksum_status" == PASS && "$manifest_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Package Build Report
Required Assets: $assets_status
Manifest: $manifest_status
Package Artifact: $package_status
Checksum Artifact: $checksum_status
Package: $PACKAGE
Checksum: $CHECKSUM
Package Build: $overall
REPORT_BODY

echo "Package Build: $overall"
[[ "$overall" == PASS ]]
