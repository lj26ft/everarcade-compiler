#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNTIME_DIR="$ROOT/deployment/evernode/runtime"
cd "$RUNTIME_DIR"

for name in runtime world deployment; do
  input_dir="package-input/$name"
  test -d "$input_dir"
  tar --sort=name --owner=0 --group=0 --numeric-owner --mtime='UTC 2026-05-31' -cf - -C "$input_dir" . | gzip -n > "arena-vanguard-$name.tar.gz"
  sha256sum "arena-vanguard-$name.tar.gz" > "arena-vanguard-$name.tar.gz.sha256"
  hash="$(cut -d' ' -f1 "arena-vanguard-$name.tar.gz.sha256")"
  printf 'everarcade-offline-signature:%s:%s\n' "arena-vanguard-$name.tar.gz" "$hash" > "arena-vanguard-$name.tar.gz.sig"
  printf '{"artifact":"arena-vanguard-%s.tar.gz","sha256":"%s","signature":"everarcade-offline-signature","reproducible":true}\n' "$name" "$hash" > "arena-vanguard-$name.receipt.json"
done

echo "evernode package artifacts generated in $RUNTIME_DIR"
