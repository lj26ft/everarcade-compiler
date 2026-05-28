#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
mkdir -p runtime/logs deployment/reports
log="runtime/logs/validation_cleanup.log"
{
  echo "validation_cleanup=started"
  du -sm target runtime/logs 2>/dev/null || true
} > "$log"
rm -rf target/debug/incremental target/tmp target/.rustc_info.json 2>/dev/null || true
find runtime/logs -type f -name '*.log' -size +20M -delete 2>/dev/null || true
{
  echo "validation_cleanup=completed"
  du -sm target runtime/logs 2>/dev/null || true
} | tee -a "$log"
