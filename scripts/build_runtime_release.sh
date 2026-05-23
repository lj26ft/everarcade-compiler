#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
STAGE_DIR="$DIST_DIR/everarcade-runtime"
TARGET_TRIPLE="${TARGET_TRIPLE:-x86_64-unknown-linux-gnu}"
RELEASE_VERSION="${RELEASE_VERSION:-0.1.0}"
MODE="dev"
if [[ "${1:-}" == "--release" ]]; then
  MODE="release"
fi

if [[ "$MODE" == "release" && -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  echo "ERROR: SOURCE_DATE_EPOCH is required in release mode" >&2
  exit 1
fi

if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  echo "WARN: SOURCE_DATE_EPOCH not set; using deterministic dev fallback timestamp" >&2
fi
BUILD_TIMESTAMP="${SOURCE_DATE_EPOCH:-1700000000}"

"$ROOT_DIR/scripts/preflight_vendor.sh"
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"/{bin,logs,docs,scripts,runtime/{config,games,worlds,replay,archives,manifests,state}}
cargo build --release -p everarcade-host -p everarcade-cli
cp "$ROOT_DIR/target/release/everarcade-host" "$STAGE_DIR/bin/everarcade-host"
cp "$ROOT_DIR/target/release/everarcade" "$STAGE_DIR/bin/everarcade-cli"
cp "$ROOT_DIR/target/release/everarcade" "$STAGE_DIR/bin/everarcade-validator"
cp "$ROOT_DIR"/scripts/{bootstrap.sh,start.sh,validate.sh,export-diagnostics.sh,shutdown.sh} "$STAGE_DIR/scripts/"
chmod +x "$STAGE_DIR/scripts/"*.sh
cp "$ROOT_DIR"/docs/{runtime-release-model.md,evernode-runtime-model.md,linux-vm-bootstrap.md} "$STAGE_DIR/docs/"
cp "$ROOT_DIR"/runtime/config/*.toml "$STAGE_DIR/runtime/config/"
echo "$RELEASE_VERSION" > "$STAGE_DIR/VERSION"
echo "everarcade-runtime" > "$STAGE_DIR/RELEASE_ROOT"
python3 - <<'PY' "$STAGE_DIR" "$RELEASE_VERSION" "$BUILD_TIMESTAMP" "$TARGET_TRIPLE"
import hashlib, json, pathlib, subprocess, sys
stage=pathlib.Path(sys.argv[1]); ver=sys.argv[2]; ts=sys.argv[3]; target=sys.argv[4]
def sha(p):
 h=hashlib.sha256(); h.update(p.read_bytes()); return h.hexdigest()
bs={f.name:sha(f) for f in sorted((stage/'bin').iterdir()) if f.is_file()}
files=[f"{p.relative_to(stage).as_posix()}:{sha(p)}" for p in sorted((stage/'runtime').rglob('*')) if p.is_file()]
runtime_hash=hashlib.sha256('\n'.join(files).encode()).hexdigest()
git_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
rust_version=subprocess.check_output(['rustc','--version'],text=True).strip()
manifest={"release_version":ver,"build_timestamp":ts,"git_commit":git_commit,"rust_version":rust_version,"target_triple":target,"binary_hashes":bs,"runtime_hash":runtime_hash,"manifest_hash":"","deterministic_mode":True}
payload=json.dumps(manifest,sort_keys=True,separators=(',',':')).encode(); manifest['manifest_hash']=hashlib.sha256(payload).hexdigest()
(stage/'MANIFEST.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n")
PY
mkdir -p "$DIST_DIR"
TARBALL="$DIST_DIR/everarcade-runtime-linux-x86_64.tar.gz"
tar --sort=name --mtime='UTC 2020-01-01' --owner=0 --group=0 --numeric-owner -czf "$TARBALL" -C "$DIST_DIR" everarcade-runtime
(cd "$DIST_DIR" && sha256sum "$(basename "$TARBALL")" > SHA256SUMS && cp everarcade-runtime/MANIFEST.json MANIFEST.json)
echo "built $TARBALL"
