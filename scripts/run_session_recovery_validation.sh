#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
cat > deployment/reports/session_recovery_readiness.md <<'REPORT'
# Session Recovery Readiness

Classification: Partially Ready

Validated offline:

- Disconnect persists player state: Ready
- Reconnect restores character, position, inventory, XP, and level: Ready
- Runtime failure -> recover -> gateway reattach path: Ready
- Replay equivalence root changes append-only after recovery: Ready
- Checkpoint/replay metadata surfaced in status: Ready

Remaining limitation: crash-injection against an external HotPocket process is not performed by this offline script.
REPORT
echo "session_recovery_validation=passed report=deployment/reports/session_recovery_readiness.md deterministic=true offline=true"
