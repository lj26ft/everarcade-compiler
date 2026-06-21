# EverArcade Operator Stdout Protocol (v0.1.0)

Automation should parse **stdout** for machine-readable success lines.
Human-oriented diagnostics and errors should use **stderr**.

## Stable success lines
- `init=ok state=/path`
- `fixture=ok output=/tmp/package.bin package_root=<hex> replay_root=<hex> checkpoint_root=<hex>`
- `receipt=<hex>`
- `verify=ok`
- `status node_name=<name> last_receipt_root=<hex|none> last_checkpoint_root=<hex|none> anchor_queue=<n>`
- `deploy_proof=ok manifest=/path/to/deployment-manifest.json receipt=<hex> checkpoint=<hex> xrpl_anchor=<id> ipfs_manifest=<cid>`
- `doctor=ok`

## Notes
- Additional stdout lines may exist, but automation should key off the stable prefixes above.
- Failure exits are non-zero and should include actionable error text on stderr.
