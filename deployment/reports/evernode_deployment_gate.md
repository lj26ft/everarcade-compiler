# EverNode Launch Gate

| Domain | Classification | Evidence |
| --- | --- | --- |
| generated package/runtime gate | Ready | Runtime, world, and deployment packages are generated with deterministic manifests, hashes, signatures, receipts, and package validation. |
| live EverNode deployment | Partially Ready | Deployment manifests, recovery checks, and local/loopback federation gates are validated; no live EverNode production deployment is performed by this repository. |
| unsupported live operations | Not Ready | Operations outside the documented deploy/start/stop/restart/recover/verify surface remain rejected and are not certified for live production use. |
| operator tooling | Partially Ready | Operator guides and validation scripts exist; production service manager integration and host operations remain operator-owned. |
| load validation | Ready | Four-node loopback TCP federation load gate validates balanced deterministic message load within capacity. |

## Gate Boundaries
- Ready status is limited to the validated generated package/runtime gate and deterministic validation artifacts.
- Live EverNode deployment remains Partially Ready until an actual production deployment is performed and evidenced.
- Unsupported live operations remain Not Ready and must not be treated as production-ready behavior.
- Generated `.tar.gz` and `.sig` files are not source controlled; rebuild with `scripts/build_evernode_packages.sh` and validate `deployment/evernode/runtime/packages.sha256`.
