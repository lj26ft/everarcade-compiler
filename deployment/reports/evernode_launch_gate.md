# EverNode Launch Gate

| Domain | Classification | Evidence |
| --- | --- | --- |
| runtime | Ready | Runtime package manifest and tarball exist with hash, signature, and receipt. |
| deployment | Ready | Deployment manifest verifies package, deployment, replay continuity, and checkpoint continuity. |
| recovery | Ready | Crash, restart, checkpoint restore, and world restore scenarios preserve roots. |
| federation | Ready | Node A and Node B deployment join, checkpoint sync, replay sync, and recovery are validated. |
| operator tooling | Partially Ready | Operator guides and validation scripts exist; production service manager integration remains operator-owned. |
| load validation | Ready | Four-node TCP federation load gate validates balanced deterministic message load within capacity. |
