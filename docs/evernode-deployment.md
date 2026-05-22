# Evernode Deployment Baseline

This baseline converts the validated development runtime into an operator-installable sovereign runtime for `everarcade-host`.

## Release Package (Reproducible)

Build and package:

```bash
bash scripts/release_package.sh dist
```

Artifacts:
- `dist/everarcade-host-operator-<utc>.tar.gz`
- `dist/everarcade-host-operator-<utc>.sha256`

Package content:
- host binary (`bin/everarcade-host`)
- operator templates (`deploy/templates/*.toml`)
- Linux service baseline (`deploy/systemd/everarcade-host.service`)
- manifest scaffold (`deploy/manifests/evernode-deployment.manifest.json`)
- operator docs

## Operator Config Profiles

- Local: `deploy/templates/operator.local.toml`
- Testnet anchor profile: `deploy/templates/operator.testnet.toml`
- Evernode/live profile: `deploy/templates/operator.evernode.toml`

## Linux VM Runner (systemd style)

1. Install package at `/opt/everarcade-host`.
2. Copy `deploy/systemd/everarcade-host.service` to `/etc/systemd/system/`.
3. Review environment flags to keep live integrations explicit.
4. Enable/start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now everarcade-host.service
sudo systemctl status everarcade-host.service
```

## Deployment Proof + Manifest Output

`deploy-proof` now emits:
- execution chain proof line
- `deployment-manifest=<path>` with machine-readable output

Live rails remain behind explicit flags:
- `ENABLE_XRPL_LIVE=1`
- `ENABLE_IPFS_LIVE=1`
- `ENABLE_EVERNODE_LIVE=1`

## Release Validation Script

Runs full baseline:
1. build release
2. generate package
3. run deploy proof
4. verify artifacts exist

```bash
bash scripts/release_validate_fresh_vm.sh
```


Notes:
- `runtime/...` is the canonical generated runtime layout for validation.
- `.everarcade-dev` is legacy and not the primary runtime contract.
- HTTPS + PAT is only needed when cloning a private repository; local validation itself does not require credentials.
