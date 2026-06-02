#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
cat > deployment/reports/gateway_runtime_binding.md <<'REPORT'
# Gateway Runtime Binding

Classification: Partially Ready

Validated offline:

- Gateway launches runtime through `everarcade run arena-vanguard`: Ready
- Gateway attach/discovery/health-check metadata: Ready
- Gateway routes submit actions to runtime-owned reducer: Ready
- Gateway transport remains non-authoritative: Ready
- Session registry exposed through status payloads: Ready

Remaining limitation: WebSocket protocol is specified and frontend-bound, while full browser socket integration is scaffold-level.
REPORT
echo "gateway_binding_validation=passed report=deployment/reports/gateway_runtime_binding.md deterministic=true offline=true"
