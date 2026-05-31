#!/usr/bin/env bash
set -euo pipefail
case "${1:-verify}" in
  deploy|start|stop|restart|recover|verify)
    echo "evernode:${1:-verify}:arena-vanguard:deterministic-local-simulation"
    ;;
  *)
    echo "unsupported operation: $1" >&2
    exit 64
    ;;
esac
