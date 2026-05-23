#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/target/everarcade-profile"
mkdir -p "$OUT"
CORE_LOG="$OUT/execution-core-tests.log"
WS_LOG="$OUT/workspace-tests.log"

cargo test -p execution-core -- --nocapture | tee "$CORE_LOG"
cargo test --workspace -- --nocapture | tee "$WS_LOG"

python3 - <<'PY' "$CORE_LOG" "$WS_LOG" "$OUT/test-profile-report.json" "$OUT/test-profile-report.md"
import json, pathlib, re, sys
core=pathlib.Path(sys.argv[1]).read_text(errors='ignore').splitlines()
ws=pathlib.Path(sys.argv[2]).read_text(errors='ignore').splitlines()
pat=re.compile(r'test\s+([^\s]+)\s+\.\.\.\s+ok\s*\(([^)]+)\)')
rows=[]
for group, lines in [('execution-core',core),('workspace',ws)]:
  for ln in lines:
    m=pat.search(ln)
    if not m: continue
    name,dur=m.group(1),m.group(2)
    rows.append({'group':group,'test':name,'duration':dur})
slow=rows[-20:]
report={
  'slowest_tests':slow,
  'long_running_test_groups':['execution-core','workspace'],
  'execution_core_runtime_boundary_tests':[r for r in rows if 'runtime' in r['test'] or 'boundary' in r['test']],
  'wasm_tests':[r for r in rows if 'wasm' in r['test']],
  'replay_tests':[r for r in rows if 'replay' in r['test']],
  'receipt_state_diff_tests':[r for r in rows if 'receipt' in r['test'] or 'state_diff' in r['test']],
}
pathlib.Path(sys.argv[3]).write_text(json.dumps(report,indent=2,sort_keys=True)+'\n')
md=['# Runtime Test Profile Report','',f"Total parsed tests: {len(rows)}",'', '## Slowest Tests']
for r in slow: md.append(f"- `{r['group']}` `{r['test']}` ({r['duration']})")
pathlib.Path(sys.argv[4]).write_text('\n'.join(md)+'\n')
PY

echo "profile reports written to $OUT"
