# Evernode v0.1.0 Deployment Bundle

## Bundle contents
- Binary path: `<prefix>/bin/everarcade-host`
- State path (recommended): `/var/lib/everarcade` (or per-tenant override)
- Operator env template: `templates/evernode/operator.env`
- Manifest template: `templates/evernode/deployment-manifest.template.json`

## Runtime defaults
- `deploy-proof` defaults to dry-run behavior for external adapters (safe local mode).
- Live adapter calls are opt-in via `--profile live` plus explicit environment flags.

## Network assumptions (operator baseline)
- Expected inbound service port: `7007/tcp` (`serve --bind 0.0.0.0:7007`)
- Sync endpoint format: `<host>:7007` (`sync --peer <host>:7007`)

## Stdout protocol
Automation should parse stdout according to `docs/stdout-protocol.md`.
