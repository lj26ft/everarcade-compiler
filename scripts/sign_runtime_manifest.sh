#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/runtime/manifests/runtime-release-manifest.json"
SIG="$ROOT/runtime/manifests/runtime-release-manifest.sig"
KEY_DIR="$ROOT/runtime/manifests/keys"
PRIVATE_KEY="${RUNTIME_MANIFEST_SIGNING_KEY:-$KEY_DIR/runtime-manifest-private.pem}"
PUBLIC_KEY="${RUNTIME_MANIFEST_PUBLIC_KEY:-$KEY_DIR/runtime-manifest-public.pem}"

mkdir -p "$KEY_DIR"
[[ -f "$MANIFEST" ]] || bash "$ROOT/scripts/hash_runtime_artifacts.sh"

if [[ ! -f "$PRIVATE_KEY" || ! -f "$PUBLIC_KEY" ]]; then
  openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -out "$PRIVATE_KEY" >/dev/null 2>&1
  openssl rsa -pubout -in "$PRIVATE_KEY" -out "$PUBLIC_KEY" >/dev/null 2>&1
fi

openssl dgst -sha256 -sign "$PRIVATE_KEY" -out "$SIG" "$MANIFEST"
echo "sign_runtime_manifest=ok signature=$SIG public_key=$PUBLIC_KEY"
