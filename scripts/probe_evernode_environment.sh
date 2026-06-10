#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_environment_probe_report.txt"
mkdir -p "$REPORT_DIR"
probe_cmd() { if command -v "$1" >/dev/null 2>&1; then printf 'yes (%s)' "$(command -v "$1")"; else printf 'no'; fi; }
{
  echo "Evernode Environment Probe Report"
  echo "Generated At: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "Hostname: $(hostname 2>/dev/null || echo unavailable)"
  echo "Kernel: $(uname -a 2>/dev/null || echo unavailable)"
  echo "Working Directory: $ROOT"
  echo
  echo "Disk:"
  df -h "$ROOT" 2>/dev/null || true
  echo
  echo "Memory:"
  free -m 2>/dev/null || awk '/MemTotal|MemFree|MemAvailable/ {print}' /proc/meminfo 2>/dev/null || true
  echo
  echo "CPU:"
  (nproc 2>/dev/null && awk -F: '/model name|cpu cores/ {gsub(/^ /,"",$2); print $1 ": " $2; if (++c>=4) exit}' /proc/cpuinfo 2>/dev/null) || true
  echo
  echo "Ports:"
  (ss -tulpen 2>/dev/null || netstat -tulpen 2>/dev/null || echo "port listing unavailable") | sed -n '1,80p'
  echo
  echo "Network Interfaces:"
  (ip addr show 2>/dev/null || ifconfig -a 2>/dev/null || echo "interface listing unavailable") | sed -n '1,120p'
  echo
  echo "Process Limits:"
  ulimit -a
  echo
  echo "Filesystem Permissions:"
  touch "$REPORT_DIR/.probe-write-test" && rm -f "$REPORT_DIR/.probe-write-test" && echo "reports writable: yes" || echo "reports writable: no"
  touch "$ROOT/.probe-write-test" && rm -f "$ROOT/.probe-write-test" && echo "repo root writable: yes" || echo "repo root writable: no"
  echo
  echo "Available Tools:"
  for tool in bash node npm curl tar gzip sha256sum ps awk sed df free ss ip netstat python3; do printf '%s: ' "$tool"; probe_cmd "$tool"; echo; done
  echo
  echo "Environment Probe: PASS"
} > "$REPORT"
cat "$REPORT"
