#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
python3 - "$ROOT" "${1:-}" <<'PY'
import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); only=(sys.argv[2]=='--hash-only')
manifest=json.loads((root/'MANIFEST.json').read_text())
for name,h in manifest['binary_hashes'].items():
 p=root/'bin'/name
 if hashlib.sha256(p.read_bytes()).hexdigest()!=h: raise SystemExit(f'binary hash mismatch: {name}')
if not only:
 for f in ['runtime.toml','federation.toml','storage.toml','replay.toml','topology.toml','evernode.toml']:
  (root/'runtime'/'config'/f).read_text()
print('ok')
PY
