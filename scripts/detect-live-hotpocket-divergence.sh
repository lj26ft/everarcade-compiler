#!/usr/bin/env bash
set -euo pipefail
ROOT="${EVERARCADE_REPO_ROOT:-$(pwd)}"
REPORT_DIR="$ROOT/reports/live-hotpocket-cluster"
"$ROOT/scripts/compare-live-hotpocket-roots.sh" >/tmp/live-hotpocket-root-compare.txt || { cat /tmp/live-hotpocket-root-compare.txt >&2; exit 1; }
cat /tmp/live-hotpocket-root-compare.txt
node --input-type=module - "$REPORT_DIR" <<'NODE'
import { readFileSync } from 'node:fs'; import { join } from 'node:path';
const divergence=JSON.parse(readFileSync(join(process.argv[2],'divergence.json'),'utf8'));
if(divergence.length){ console.error(JSON.stringify(divergence,null,2)); process.exit(1); }
console.log('No live HotPocket divergence detected.');
NODE
