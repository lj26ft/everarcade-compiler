#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/runtime/manifests/runtime-release-manifest.json"
SIG="$ROOT/runtime/manifests/runtime-release-manifest.sig"
PUBLIC_KEY="${RUNTIME_MANIFEST_PUBLIC_KEY:-$ROOT/runtime/manifests/keys/runtime-manifest-public.pem}"

[[ -f "$MANIFEST" ]] || { echo "missing manifest $MANIFEST" >&2; exit 1; }
[[ -f "$SIG" ]] || { echo "missing signature $SIG" >&2; exit 1; }
[[ -f "$PUBLIC_KEY" ]] || { echo "missing public key $PUBLIC_KEY" >&2; exit 1; }

openssl dgst -sha256 -verify "$PUBLIC_KEY" -signature "$SIG" "$MANIFEST" >/dev/null

echo "verify_runtime_manifest_signature=ok"
