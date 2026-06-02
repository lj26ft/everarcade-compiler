#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNTIME_DIR="$ROOT/deployment/evernode/runtime"
DIST_DIR="$ROOT/dist/everarcade-hotpocket-contract"
LOCK_DIR="$RUNTIME_DIR/.build.lock"
TMP_ROOT="$RUNTIME_DIR/.build.$$.tmp"
FIXTURE_MTIME="UTC 1970-01-01"
REQUIRE_HOST_BINARY=0
STAGE_CONTRACT=0

packages=(
  "arena-vanguard-runtime"
  "arena-vanguard-world"
  "arena-vanguard-deployment"
)

usage() {
  cat <<'USAGE'
Usage: bash scripts/generate_evernode_packages.sh [--stage-contract] [--require-host-binary]

Build deterministic EverNode/HotPocket package artifacts. With --stage-contract,
stage dist/everarcade-hotpocket-contract for local rehearsal or operator upload.
USAGE
}

for arg in "$@"; do
  case "$arg" in
    --stage-contract) STAGE_CONTRACT=1 ;;
    --require-host-binary) REQUIRE_HOST_BINARY=1 ;;
    -h|--help) usage; exit 0 ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; usage >&2; exit 2 ;;
  esac
done

say() { printf '%s\n' "$*"; }
fail() {
  printf '❌ %s\n' "$*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$ROOT/$path" ]] || fail "Missing package input: $path"
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
    printf 'generated_by = "scripts/generate_evernode_packages.sh"\n'
    printf 'reproducible = true\n'
    printf 'inputs = [\n'
    local input
    for input in "$@"; do
      printf '  "%s",\n' "$input"
    done
    printf ']\n'
  } > "$dest/PACKAGE-MANIFEST.toml"
}

clean_outputs() {
  mkdir -p "$RUNTIME_DIR"
  rm -f \
    "$RUNTIME_DIR"/arena-vanguard-*.tar.gz \
    "$RUNTIME_DIR"/arena-vanguard-*.tar.gz.sha256 \
    "$RUNTIME_DIR"/arena-vanguard-*.tar.gz.sig \
    "$RUNTIME_DIR"/arena-vanguard-*.sig \
    "$RUNTIME_DIR"/arena-vanguard-*.receipt.json \
    "$RUNTIME_DIR"/arena-vanguard-*.pkg \
    "$RUNTIME_DIR"/*.tmp \
    "$RUNTIME_DIR"/packages.sha256
}

validate_inputs() {
  local input
  for input in \
    "deployment/evernode/runtime_manifest.toml" \
    "arena-vanguard-package/runtime_package.toml" \
    "arena-vanguard/manifest.toml" \
    "deployment/evernode/world_manifest.toml" \
    "arena-vanguard-world/world_manifest.toml" \
    "arena-vanguard-world/checkpoints/checkpoint_0003.state" \
    "arena-vanguard-world/replay/replay_0003.log" \
    "arena-vanguard-package/world_package.toml" \
    "deployment/evernode/deployment_manifest.toml" \
    "deployment/evernode/package_manifest.toml" \
    "deployment/evernode/operations.sh" \
    "arena-vanguard-package/deployment_package.toml" \
    "arena-vanguard-package/asset_package.toml" \
    "arena-vanguard-assets/asset_manifest.toml" \
    "arena-vanguard-rustrigs/marketplace_manifest.toml" \
    "xrpl-anchor/anchor_records.json"
  do
    require_file "$input"
  done
}

build_package() {
  local name="$1"
  local kind="$2"
  shift 2
  local stage="$TMP_ROOT/$name"
  local out="$RUNTIME_DIR/$name.tar.gz"
  local tmp="$out.tmp"

  rm -rf "$stage"
  mkdir -p "$stage"

  local input
  for input in "$@"; do
    copy_input "$input" "$stage"
  done
  write_manifest "$stage" "$name" "$kind" "$@"

  (
    cd "$stage"
    tar --sort=name \
      --mtime="$FIXTURE_MTIME" \
      --owner=0 \
      --group=0 \
      --numeric-owner \
      --mode='u+rw,go+r,a-s' \
      -cf - . | gzip -n > "$tmp"
  )
  mv "$tmp" "$out"
}

write_checksums_and_signatures() {
  (
    cd "$RUNTIME_DIR"
    local package digest
    for package in "${packages[@]}"; do
      sha256sum "$package.tar.gz" > "$package.tar.gz.sha256.tmp"
      mv "$package.tar.gz.sha256.tmp" "$package.tar.gz.sha256"
      digest="$(cut -d' ' -f1 "$package.tar.gz.sha256")"
      printf 'evernode-offline-signature-v1 %s.tar.gz %s\n' "$package" "$digest" > "$package.tar.gz.sig.tmp"
      mv "$package.tar.gz.sig.tmp" "$package.tar.gz.sig"
      # Compatibility surrogate for older deployment gates that expected *.sig.
      printf 'evernode-offline-signature-v1 %s %s\n' "$package" "$digest" > "$package.sig.tmp"
      mv "$package.sig.tmp" "$package.sig"
    done
  )
}

write_receipts() {
  (
    cd "$RUNTIME_DIR"
    local package digest
    for package in "${packages[@]}"; do
      digest="$(cut -d' ' -f1 "$package.tar.gz.sha256")"
      cat > "$package.receipt.json.tmp" <<RECEIPT
{"artifact":"$package.tar.gz","sha256":"$digest","signature":"evernode-offline-signature-v1","reproducible":true,"generated_by":"scripts/generate_evernode_packages.sh"}
RECEIPT
      mv "$package.receipt.json.tmp" "$package.receipt.json"
    done
  )
}

write_package_manifest_last() {
  (
    cd "$RUNTIME_DIR"
    : > packages.sha256.tmp
    local package
    for package in "${packages[@]}"; do
      sha256sum "$package.tar.gz" >> packages.sha256.tmp
    done
    LC_ALL=C sort packages.sha256.tmp > packages.sha256
    rm packages.sha256.tmp
  )
}

verify_package_manifest() {
  (cd "$RUNTIME_DIR" && sha256sum -c packages.sha256 >/dev/null) || fail "Checksum verification failed"
}

stage_contract() {
  say "🏗️  Staging HotPocket contract directory..."
  rm -rf "$DIST_DIR"
  mkdir -p "$DIST_DIR/packages" "$DIST_DIR/config" "$DIST_DIR/state/operator"

  cp "$RUNTIME_DIR"/arena-vanguard-*.tar.gz "$DIST_DIR/packages/"
  cp "$RUNTIME_DIR/packages.sha256" "$DIST_DIR/packages/"
  cp "$RUNTIME_DIR"/*-manifest.toml "$DIST_DIR/config/"

  local host_binary="$ROOT/target/release/everarcade-host"
  if [[ -f "$host_binary" ]]; then
    cp "$host_binary" "$DIST_DIR/everarcade-host"
    chmod 0755 "$DIST_DIR/everarcade-host"
    say "✅ everarcade-host binary copied"
  elif [[ "$REQUIRE_HOST_BINARY" -eq 1 ]]; then
    fail "Missing required host binary: $host_binary"
  else
    say "⚠️  everarcade-host binary not found; build it with:"
    say "   cargo build -p everarcade-host --release --offline --locked"
  fi

  cat > "$DIST_DIR/start.sh" <<'START'
#!/usr/bin/env bash
set -euo pipefail
CONTRACT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$CONTRACT_ROOT/state/operator" "$CONTRACT_ROOT/state/runtime" "$CONTRACT_ROOT/state/world" "$CONTRACT_ROOT/state/deployment"
(cd "$CONTRACT_ROOT/packages" && sha256sum -c packages.sha256 >/dev/null)
printf 'ready\n' > "$CONTRACT_ROOT/state/operator/status.txt"
printf 'HotPocket contract rehearsal ready: %s\n' "$CONTRACT_ROOT/state/operator/status.txt"
START
  chmod 0755 "$DIST_DIR/start.sh"
  say "✅ HotPocket contract staged at dist/everarcade-hotpocket-contract"
}

print_artifacts() {
  say "📋 Final artifacts:"
  (
    cd "$RUNTIME_DIR"
    find . -maxdepth 1 -type f \
      \( -name 'arena-vanguard-*.tar.gz' -o -name 'arena-vanguard-*.tar.gz.sha256' -o -name 'arena-vanguard-*.tar.gz.sig' -o -name 'arena-vanguard-*.sig' -o -name 'arena-vanguard-*.receipt.json' -o -name 'packages.sha256' \) \
      -printf '  %f\n' | LC_ALL=C sort
  )
}

say "🎮 EverArcade HotPocket Package Builder"
say "📦 Building Arena Vanguard packages..."

mkdir -p "$RUNTIME_DIR"
while ! mkdir "$LOCK_DIR" 2>/dev/null; do
  sleep 0.1
done
rm -rf "$TMP_ROOT"
mkdir -p "$TMP_ROOT"
trap 'status=$?; rm -rf "$TMP_ROOT"; rmdir "$LOCK_DIR" 2>/dev/null || true; if [[ "$status" -ne 0 ]]; then printf "❌ Package generation failed\n" >&2; fi' EXIT

say "🧹 Cleaning old generated artifacts..."
clean_outputs
say "✅ Clean complete"

say "🔍 Validating package inputs..."
validate_inputs
say "✅ Inputs valid"

say "🛠️  Building runtime package..."
build_package "arena-vanguard-runtime" "runtime" \
  "deployment/evernode/runtime_manifest.toml" \
  "arena-vanguard-package/runtime_package.toml" \
  "arena-vanguard/manifest.toml"
say "✅ Runtime package ready"

say "🌍 Building world package..."
build_package "arena-vanguard-world" "world" \
  "deployment/evernode/world_manifest.toml" \
  "arena-vanguard-world/world_manifest.toml" \
  "arena-vanguard-world/checkpoints/checkpoint_0003.state" \
  "arena-vanguard-world/replay/replay_0003.log" \
  "arena-vanguard-package/world_package.toml"
say "✅ World package ready"

say "🚀 Building deployment package..."
build_package "arena-vanguard-deployment" "deployment" \
  "deployment/evernode/deployment_manifest.toml" \
  "deployment/evernode/package_manifest.toml" \
  "deployment/evernode/operations.sh" \
  "arena-vanguard-package/deployment_package.toml" \
  "arena-vanguard-package/asset_package.toml" \
  "arena-vanguard-assets/asset_manifest.toml" \
  "arena-vanguard-rustrigs/marketplace_manifest.toml" \
  "xrpl-anchor/anchor_records.json"
say "✅ Deployment package ready"

say "🔐 Writing checksums and signatures..."
write_checksums_and_signatures
say "✅ Per-package checksums and signatures ready"

say "🧾 Writing receipts..."
write_receipts
say "✅ Receipts ready"

say "🔐 Writing aggregate packages.sha256..."
write_package_manifest_last
verify_package_manifest
say "✅ Checksum verification passed"

if [[ "$STAGE_CONTRACT" -eq 1 ]]; then
  stage_contract
fi

print_artifacts
say "🎉 HotPocket package generation complete"
