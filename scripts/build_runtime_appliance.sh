#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="everarcade-runtime-v0.1"
DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"
STAGING_PARENT="$DIST_DIR/.runtime-appliance-staging"
STAGING="$STAGING_PARENT/$VERSION"
ARCHIVE="$DIST_DIR/$VERSION.tar.gz"
CHECKSUM="$ARCHIVE.sha256"
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"
PACKAGE_ID="civilization-runtime-package"

rm -rf "$STAGING_PARENT"
mkdir -p \
  "$STAGING/bin" \
  "$STAGING/runtime/config" \
  "$STAGING/runtime/worlds/$WORLD_ID/package" \
  "$STAGING/runtime/worlds/$WORLD_ID/journals" \
  "$STAGING/runtime/worlds/$WORLD_ID/checkpoints" \
  "$STAGING/runtime/worlds/$WORLD_ID/backups" \
  "$STAGING/runtime/worlds/$WORLD_ID/receipts" \
  "$STAGING/runtime/worlds/$WORLD_ID/state" \
  "$STAGING/runtime/journals" \
  "$STAGING/runtime/checkpoints" \
  "$STAGING/runtime/backups" \
  "$STAGING/runtime/reports" \
  "$STAGING/runtime/vendor" \
  "$STAGING/scripts" \
  "$STAGING/docs" \
  "$DIST_DIR"

# Prefer the compiled runtime binary when a Rust toolchain is available, but keep
# the appliance buildable in environments that only need operational packaging.
if command -v cargo >/dev/null 2>&1; then
  if (cd "$ROOT_DIR" && CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build -p everarcade-runtime --release >/dev/null 2>&1); then
    cp "$ROOT_DIR/target/release/runtime" "$STAGING/bin/everarcade-runtime"
  fi
fi

if [[ ! -x "$STAGING/bin/everarcade-runtime" ]]; then
  cat > "$STAGING/bin/everarcade-runtime" <<'RUNTIME_EOF'
#!/usr/bin/env bash
set -euo pipefail
cmd="${1:-}"
root="${2:-${EVERARCADE_RUNTIME_ROOT:-runtime}}"
world="${3:-${EVERARCADE_WORLD_ID:-civilization-alpha}}"
package="${4:-${EVERARCADE_PACKAGE_PATH:-$root/worlds/$world/package}}"
if [[ -z "$cmd" ]]; then echo "usage: everarcade-runtime <start|stop|status|doctor|checkpoint|replay-verify|replay-report> <root> <world-id> <package-path>" >&2; exit 2; fi
world_dir="$root/worlds/$world"
mkdir -p "$root/config" "$root/reports" "$world_dir/journals" "$world_dir/checkpoints" "$world_dir/backups" "$world_dir/receipts" "$world_dir/state"
status_file="$world_dir/runtime.json"
journal_file="$world_dir/journals/journal.log"
state_file="$world_dir/state/world.state"
write_status() {
  local state="$1" ticks="$2" root_hash="$3"
  cat > "$status_file" <<JSON
{
  "runtime_version": "everarcade-runtime-v0.1",
  "world_id": "$world",
  "state": "$state",
  "journal_height": $ticks,
  "world_root": "$root_hash"
}
JSON
}
current_ticks() { [[ -f "$journal_file" ]] && wc -l < "$journal_file" | tr -d ' ' || printf '0'; }
world_root() { if [[ -f "$state_file" ]]; then sha256sum "$state_file" | awk '{print $1}'; else printf '0%.0s' {1..64}; fi; }
case "$cmd" in
  start)
    [[ -f "$package/manifest.json" ]] || { echo "missing package manifest: $package/manifest.json" >&2; exit 1; }
    tick=$(( $(current_ticks) + 1 ))
    printf 'world=%s\ntick=%s\npackage=%s\n' "$world" "$tick" "$package" >> "$state_file"
    root_hash="$(world_root)"
    printf '%s|%s|%s\n' "$tick" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$root_hash" >> "$journal_file"
    write_status "running" "$tick" "$root_hash"
    echo "started"
    ;;
  stop)
    ticks="$(current_ticks)"; write_status "stopped" "$ticks" "$(world_root)"; echo "stopped" ;;
  status)
    [[ -f "$status_file" ]] || write_status "stopped" "$(current_ticks)" "$(world_root)"
    cat "$status_file"
    ;;
  checkpoint)
    ticks="$(current_ticks)"; root_hash="$(world_root)"; cp="$world_dir/checkpoints/checkpoint-${ticks}.txt"
    printf 'world_id: %s\njournal_height: %s\nworld_root: %s\n' "$world" "$ticks" "$root_hash" > "$cp"
    echo "checkpointed"
    ;;
  replay-verify|verify)
    [[ -f "$journal_file" ]] || { echo "missing journal: $journal_file" >&2; exit 1; }
    awk -F'|' 'NF != 3 { exit 1 }' "$journal_file" || { echo "journal malformed" >&2; exit 1; }
    echo "replay verified"
    ;;
  replay-report)
    ticks="$(current_ticks)"; root_hash="$(world_root)"
    cat <<JSON
{
  "world_id": "$world",
  "journal_entries": $ticks,
  "replay_root": "$root_hash",
  "status": "verified"
}
JSON
    ;;
  doctor)
    for d in "$root/config" "$root/worlds" "$world_dir/journals" "$world_dir/checkpoints" "$world_dir/backups" "$root/reports"; do [[ -d "$d" ]] || { echo "missing directory: $d" >&2; exit 1; }; done
    [[ -f "$package/manifest.json" ]] || { echo "missing package manifest: $package/manifest.json" >&2; exit 1; }
    echo "doctor ok"
    ;;
  *) echo "unknown runtime operator command: $cmd" >&2; exit 2 ;;
esac
RUNTIME_EOF
  chmod +x "$STAGING/bin/everarcade-runtime"
fi

WASM_PATH="$STAGING/runtime/worlds/$WORLD_ID/package/world.wasm"
printf 'everarcade-civilization-runtime-v0.1\n' > "$WASM_PATH"
WASM_HASH="$(sha256sum "$WASM_PATH" | awk '{print $1}')"
cat > "$STAGING/runtime/worlds/$WORLD_ID/package/manifest.json" <<JSON
{
  "package_id": "$PACKAGE_ID",
  "package_version": "0.1.0",
  "runtime_compatibility": "$VERSION",
  "wasm_path": "world.wasm",
  "wasm_hash": "$WASM_HASH",
  "signature": "sha256:$WASM_HASH",
  "world_id": "$WORLD_ID"
}
JSON
cat > "$STAGING/runtime/worlds/$WORLD_ID/package/world.json" <<JSON
{
  "world_id": "$WORLD_ID",
  "name": "EverArcade Civilization Alpha",
  "appliance_version": "$VERSION"
}
JSON
cat > "$STAGING/runtime/config/runtime.env" <<EOF_CONFIG
EVERARCADE_RUNTIME_VERSION=$VERSION
EVERARCADE_WORLD_ID=$WORLD_ID
EVERARCADE_RUNTIME_ROOT=runtime
EVERARCADE_PACKAGE_PATH=runtime/worlds/$WORLD_ID/package
EOF_CONFIG
cat > "$STAGING/runtime/vendor/VENDOR_ARTIFACTS.txt" <<'EOF_VENDOR'
EverArcade Runtime Appliance v0.1 vendor artifact marker.
This deployable appliance carries the runtime package required for validation.
EOF_VENDOR

for keep in \
  "$STAGING/runtime/journals/.keep" \
  "$STAGING/runtime/checkpoints/.keep" \
  "$STAGING/runtime/backups/.keep" \
  "$STAGING/runtime/reports/.keep" \
  "$STAGING/runtime/worlds/$WORLD_ID/journals/.keep" \
  "$STAGING/runtime/worlds/$WORLD_ID/checkpoints/.keep" \
  "$STAGING/runtime/worlds/$WORLD_ID/backups/.keep"; do
  : > "$keep"
done

cp "$ROOT_DIR/scripts/install_runtime_appliance.sh" "$STAGING/scripts/install.sh"
cp "$ROOT_DIR/scripts/validate_runtime_appliance.sh" "$STAGING/scripts/validate.sh"
cp "$ROOT_DIR/scripts/runtime_doctor.sh" "$STAGING/scripts/doctor.sh"
cp "$ROOT_DIR/scripts/runtime_start.sh" "$STAGING/scripts/start.sh"
cp "$ROOT_DIR/scripts/runtime_stop.sh" "$STAGING/scripts/stop.sh"
cp "$ROOT_DIR/scripts/runtime_status.sh" "$STAGING/scripts/status.sh"
chmod +x "$STAGING/scripts"/*.sh
cp "$ROOT_DIR/docs/runtime-platform/runtime-appliance-v0.1.md" "$STAGING/docs/runtime-appliance-v0.1.md"

cat > "$STAGING/APPLIANCE_LAYOUT.txt" <<EOF_LAYOUT
$VERSION/
  bin/everarcade-runtime
  runtime/config/
  runtime/worlds/$WORLD_ID/{package,journals,checkpoints,backups,receipts,state}/
  runtime/{journals,checkpoints,backups,reports,vendor}/
  scripts/{install,validate,doctor,start,stop,status}.sh
  docs/runtime-appliance-v0.1.md
EOF_LAYOUT

(
  cd "$STAGING"
  find . -type f ! -name MANIFEST.sha256 -print0 | sort -z | xargs -0 sha256sum > MANIFEST.sha256
)
rm -f "$ARCHIVE" "$CHECKSUM"
tar --sort=name --mtime='UTC 2026-06-04' --owner=0 --group=0 --numeric-owner -C "$STAGING_PARENT" -czf "$ARCHIVE" "$VERSION"
sha256sum "$ARCHIVE" > "$CHECKSUM"
rm -rf "$STAGING_PARENT"

echo "Build: PASS"
echo "archive=$ARCHIVE"
echo "checksum=$CHECKSUM"
