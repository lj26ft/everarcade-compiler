#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-lease-handoff-$VERSION"
DIST_DIR="$ROOT/dist"
REPORT_DIR="$ROOT/reports"
PACKAGE="$DIST_DIR/$NAME.tar.gz"
CHECKSUM="$PACKAGE.sha256"
STAGING_PARENT="$ROOT/tmp/lease-handoff-build"
STAGING="$STAGING_PARENT/$NAME"
REPORT="$REPORT_DIR/lease_handoff_build_report.txt"
MANIFEST_PATH="$STAGING/manifests/deployment-manifest.txt"
CONTENTS_PATH="$STAGING/checksums/package-files.sha256"

mkdir -p "$DIST_DIR" "$REPORT_DIR" "$STAGING_PARENT"
rm -rf "$STAGING" "$PACKAGE" "$CHECKSUM"
mkdir -p "$STAGING"

for ignore_pattern in \
  '/dist/everarcade-lease-handoff-v0.1.tar.gz' \
  '/dist/everarcade-lease-handoff-v0.1.tar.gz.sha256' \
  '/handoff/lease-install/' \
  '/tmp/lease-handoff-build/' \
  '/tmp/lease-handoff-verify/' \
  '/tmp/lease-handoff-install-verify/'; do
  grep -qxF "$ignore_pattern" "$ROOT/.git/info/exclude" || printf '%s\n' "$ignore_pattern" >> "$ROOT/.git/info/exclude"
done

required_paths=(
  evernode/lease/deployment-manifest.txt
  evernode/runtime
  evernode/node
  evernode/hotpocket
  runtime/config
  runtime/deployment
  node/config
  node/state
  hotpocket/adapter
  hotpocket/README.md
  scripts/evernode_bootstrap.sh
  scripts/evernode_start.sh
  scripts/evernode_stop.sh
  scripts/node_common.sh
  scripts/node_doctor.sh
  scripts/node_start.sh
  scripts/runtime_doctor.sh
  scripts/runtime_start.sh
  scripts/runtime_stop.sh
  scripts/runtime_status.sh
  scripts/validate_hotpocket_integration.sh
  scripts/certify_hotpocket_integration.sh
  scripts/validate_evernode_deployment.sh
  scripts/certify_evernode_deployment.sh
  docs/runtime-platform/runtime-appliance-v0.1.md
  docs/runtime-platform/protocol-node-appliance-v0.1.md
  docs/runtime-platform/hotpocket-integration-v0.1.md
  docs/runtime-platform/evernode-deployment-v0.1.md
  docs/runtime-platform/evernode-lease-handoff-v0.1.md
  handoff/docs/operator-runbook.txt
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
Evernode Lease Handoff Build Report
Required Assets: $assets_status
Package: $PACKAGE
Checksum: $CHECKSUM
Lease Handoff Build: FAIL
REPORT_BODY
  echo "Lease Handoff Build: FAIL"
  exit 1
fi

mkdir -p \
  "$STAGING/package/evernode" \
  "$STAGING/package/runtime" \
  "$STAGING/package/node" \
  "$STAGING/package/hotpocket" \
  "$STAGING/docs/runtime-platform" \
  "$STAGING/scripts" \
  "$STAGING/manifests" \
  "$STAGING/checksums" \
  "$STAGING/reports"

cp -R evernode/lease evernode/runtime evernode/node evernode/hotpocket evernode/checkpoints evernode/journals evernode/backups evernode/logs evernode/reports "$STAGING/package/evernode/"
cp -R runtime/config runtime/deployment "$STAGING/package/runtime/"
cp -R node/config node/state node/worlds node/journals node/checkpoints node/backups node/reports node/logs "$STAGING/package/node/"
cp -R hotpocket/adapter hotpocket/input hotpocket/output hotpocket/checkpoint hotpocket/replay hotpocket/settlement hotpocket/status "$STAGING/package/hotpocket/"
cp hotpocket/README.md "$STAGING/package/hotpocket/README.md"
cp \
  docs/runtime-platform/runtime-appliance-v0.1.md \
  docs/runtime-platform/protocol-node-appliance-v0.1.md \
  docs/runtime-platform/hotpocket-integration-v0.1.md \
  docs/runtime-platform/evernode-deployment-v0.1.md \
  docs/runtime-platform/evernode-lease-handoff-v0.1.md \
  "$STAGING/docs/runtime-platform/"
cp handoff/docs/operator-runbook.txt "$STAGING/docs/operator-runbook.txt"
cp \
  scripts/evernode_bootstrap.sh \
  scripts/evernode_start.sh \
  scripts/evernode_stop.sh \
  scripts/evernode_health.sh \
  scripts/validate_evernode_deployment.sh \
  scripts/certify_evernode_deployment.sh \
  scripts/node_common.sh \
  scripts/node_doctor.sh \
  scripts/node_init.sh \
  scripts/node_start.sh \
  scripts/node_checkpoint.sh \
  scripts/node_replay.sh \
  scripts/node_restore.sh \
  scripts/node_stop.sh \
  scripts/runtime_doctor.sh \
  scripts/runtime_start.sh \
  scripts/runtime_stop.sh \
  scripts/runtime_status.sh \
  scripts/validate_hotpocket_integration.sh \
  scripts/certify_hotpocket_integration.sh \
  scripts/build_lease_handoff_package.sh \
  scripts/verify_lease_handoff_package.sh \
  scripts/install_lease_handoff_package.sh \
  scripts/validate_lease_handoff_package.sh \
  scripts/certify_lease_handoff_package.sh \
  "$STAGING/scripts/"

cat > "$MANIFEST_PATH" <<MANIFEST
EverArcade Evernode Lease Handoff Package v0.1
Package: $NAME
Immutable Artifact: dist/$NAME.tar.gz
Checksum Artifact: dist/$NAME.tar.gz.sha256
Handoff Layout: package/, docs/, scripts/, manifests/, checksums/, reports/
Evernode Deployment Assets: package/evernode/
Runtime Appliance Assets: package/runtime/config, package/runtime/deployment, scripts/runtime_*.sh
Protocol Node Assets: package/node/, scripts/node_*.sh
HotPocket Assets: package/hotpocket/, scripts/validate_hotpocket_integration.sh, scripts/certify_hotpocket_integration.sh
Operator Scripts: scripts/build_lease_handoff_package.sh, scripts/verify_lease_handoff_package.sh, scripts/install_lease_handoff_package.sh, scripts/validate_lease_handoff_package.sh, scripts/certify_lease_handoff_package.sh
Documentation: docs/operator-runbook.txt, docs/runtime-platform/evernode-lease-handoff-v0.1.md
Reports Directory: reports/
Live Lease Registration: excluded
Evernode Credentials: excluded
XRPL Credentials: excluded
Xaman Credentials: excluded
Networking: disabled
Consensus: disabled
XRPL RPC: disabled
Hooks: disabled
Xaman Signing: disabled
MANIFEST
cp evernode/lease/deployment-manifest.txt "$STAGING/manifests/evernode-deployment-manifest.txt"

cat > "$STAGING/reports/README.txt" <<'REPORTS'
Generated lifecycle reports are written by build, verify, install, validate, and certify scripts.
REPORTS

find "$STAGING" -type f \( -name '*.tmp' -o -name '*.log' -o -name '*.env' \) -delete
find "$STAGING" -type f -exec chmod 0644 {} +
find "$STAGING/scripts" -type f -name '*.sh' -exec chmod 0755 {} +

(
  cd "$STAGING"
  find . -type f ! -path './checksums/package-files.sha256' -print0 \
    | sort -z \
    | xargs -0 sha256sum \
    | sed 's# ./# #' > "$CONTENTS_PATH"
)

if tar --help 2>/dev/null | grep -q -- '--sort'; then
  tar -C "$STAGING_PARENT" --sort=name --mtime='UTC 2026-01-01' --owner=0 --group=0 --numeric-owner -czf "$PACKAGE" "$NAME"
else
  tar -C "$STAGING_PARENT" -czf "$PACKAGE" "$NAME"
fi
(cd "$DIST_DIR" && sha256sum "$(basename "$PACKAGE")") > "$CHECKSUM"

package_status="FAIL"
checksum_status="FAIL"
manifest_status="FAIL"
contents_status="FAIL"
[[ -s "$PACKAGE" ]] && package_status="PASS"
[[ -s "$CHECKSUM" ]] && checksum_status="PASS"
[[ -s "$MANIFEST_PATH" ]] && manifest_status="PASS"
[[ -s "$CONTENTS_PATH" ]] && contents_status="PASS"

overall="FAIL"
if [[ "$package_status" == PASS && "$checksum_status" == PASS && "$manifest_status" == PASS && "$contents_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Lease Handoff Build Report
Required Assets: $assets_status
Manifest: $manifest_status
Content Checksums: $contents_status
Package Artifact: $package_status
Checksum Artifact: $checksum_status
Package: $PACKAGE
Checksum: $CHECKSUM
Lease Handoff Build: $overall
REPORT_BODY

echo "Lease Handoff Build: $overall"
[[ "$overall" == PASS ]]
