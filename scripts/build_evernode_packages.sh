#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNTIME_DIR="$ROOT/deployment/evernode/runtime"
LOCK_DIR="$RUNTIME_DIR/.build.lock"
TMP_ROOT="$RUNTIME_DIR/.build.$$ .tmp"
TMP_ROOT="${TMP_ROOT// /}"

packages=(
  "arena-vanguard-runtime"
  "arena-vanguard-world"
  "arena-vanguard-deployment"
)

require_file() {
  local path="$1"
  if [[ ! -f "$ROOT/$path" ]]; then
    echo "missing required packaging input: $path" >&2
    exit 1
  fi
}

copy_input() {
  local src="$1"
  local dest_root="$2"
  require_file "$src"
  mkdir -p "$dest_root/$(dirname "$src")"
  cp "$ROOT/$src" "$dest_root/$src"
}

write_manifest() {
  local dest="$1"
  local name="$2"
  local kind="$3"
  shift 3
  {
    printf 'package = "%s"\n' "$name"
    printf 'kind = "%s"\n' "$kind"
    printf 'source = "canonical EverNode deployment manifests"\n'
    printf 'generated_by = "scripts/build_evernode_packages.sh"\n'
    printf 'inputs = [\n'
    local input
    for input in "$@"; do
      printf '  "%s",\n' "$input"
    done
    printf ']\n'
  } > "$dest/PACKAGE-MANIFEST.toml"
}

build_package() {
  local name="$1"
  local kind="$2"
  shift 2
  local stage="$TMP_ROOT/$name"
  rm -rf "$stage"
  mkdir -p "$stage"

  local input
  for input in "$@"; do
    copy_input "$input" "$stage"
  done
  write_manifest "$stage" "$name" "$kind" "$@"

  local out="$RUNTIME_DIR/$name.tar.gz"
  local tmp="$out.tmp"
  (
    cd "$stage"
    tar --sort=name \
      --mtime='UTC 1970-01-01' \
      --owner=0 \
      --group=0 \
      --numeric-owner \
      -cf - . | gzip -n > "$tmp"
  )
  mv "$tmp" "$out"

  # Offline deterministic signature surrogate: bind the package digest to the
  # package name without requiring a private key or network-backed signer.
  local digest
  digest="$(sha256sum "$out" | awk '{print $1}')"
  printf 'evernode-offline-signature-v1 %s %s\n' "$name" "$digest" > "$RUNTIME_DIR/$name.sig.tmp"
  mv "$RUNTIME_DIR/$name.sig.tmp" "$RUNTIME_DIR/$name.sig"
}

mkdir -p "$RUNTIME_DIR"
while ! mkdir "$LOCK_DIR" 2>/dev/null; do
  sleep 0.1
done
rm -rf "$TMP_ROOT"
mkdir -p "$TMP_ROOT"
trap 'rm -rf "$TMP_ROOT"; rmdir "$LOCK_DIR" 2>/dev/null || true' EXIT

build_package "arena-vanguard-runtime" "runtime" \
  "deployment/evernode/runtime_manifest.toml" \
  "arena-vanguard-package/runtime_package.toml" \
  "arena-vanguard/manifest.toml"

build_package "arena-vanguard-world" "world" \
  "deployment/evernode/world_manifest.toml" \
  "arena-vanguard-world/world_manifest.toml" \
  "arena-vanguard-world/checkpoints/checkpoint_0003.state" \
  "arena-vanguard-world/replay/replay_0003.log" \
  "arena-vanguard-package/world_package.toml"

build_package "arena-vanguard-deployment" "deployment" \
  "deployment/evernode/deployment_manifest.toml" \
  "deployment/evernode/package_manifest.toml" \
  "deployment/evernode/operations.sh" \
  "arena-vanguard-package/deployment_package.toml" \
  "arena-vanguard-package/asset_package.toml" \
  "arena-vanguard-assets/asset_manifest.toml" \
  "arena-vanguard-rustrigs/marketplace_manifest.toml" \
  "xrpl-anchor/anchor_records.json"

(
  cd "$RUNTIME_DIR"
  : > packages.sha256.tmp
  for package in "${packages[@]}"; do
    sha256sum "$package.tar.gz" >> packages.sha256.tmp
  done
  LC_ALL=C sort packages.sha256.tmp > packages.sha256
  rm packages.sha256.tmp

  for package in "${packages[@]}"; do
    sha256sum -c <(grep "  $package.tar.gz$" packages.sha256) >/dev/null
    grep -Eq "^evernode-offline-signature-v1 $package [0-9a-f]{64}$" "$package.sig"
  done
)

echo "evernode packages built in deployment/evernode/runtime"
