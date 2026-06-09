# Local Game Launch Proof

The local launch path connects Creator SDK output to `everarcade-runtime` and proves that the runtime can boot a package produced from a generated game template.

## Command

```bash
node creator-sdk/cli/everarcade.mjs launch-local \
  --project /tmp/everarcade-local-launch-proof/audit-arena \
  --runtime-root /tmp/everarcade-local-launch-proof/runtime-root
```

The command ensures `dist/runtime-package` exists and then starts the runtime with:

```bash
cargo run -q -p everarcade-runtime --bin runtime -- start \
  <runtime-root> <world-id> <runtime-package-dir>
```

Expected success output:

```text
Local Runtime Launch: PASS (<world-id>)
```

The Creator CLI captures stdout, stderr, exit code, package id, world id, runtime root, and package directory in:

```text
dist/local-launch-report.json
```

## Runtime evidence files

A successful runtime start writes real runtime output under the selected runtime root:

```text
runtime-root/
  reports/runtime_start_report.json
  worlds/<world-id>/sessions/session-0001.json
  worlds/<world-id>/projections/projection-0001.json
```

Each evidence file includes the required boot fields:

- `world_id`
- `package_id`
- `package_hash`
- `runtime_version`
- `status`
- `classification`

Projection evidence also includes:

```json
{
  "non_authoritative_projection": true
}
```

## Validation and certification

Run the full proof flow with:

```bash
bash scripts/validate_local_game_launch.sh
bash scripts/certify_local_game_launch.sh
```

The validation script creates an arena project, builds it, tests it, packages it, launches it through `everarcade-runtime`, verifies package and evidence files, and writes:

```text
reports/local_game_launch_validation_report.txt
```

The certification script consumes the validation report and writes:

```text
reports/local_game_launch_certification_report.txt
```

Expected certification result:

```text
Local Game Launch: RUNTIME BOOT PROVEN
```

## What is proven

This proves runtime boot from a creator-generated package. The runtime loader accepted the generated manifest, placeholder WASM bytes, hash, signature placeholder, and world metadata, and then wrote boot/session/projection evidence from the runtime start path.

## What is not proven

This does not yet prove real WASM game execution, an interactive game loop, authoritative multiplayer, renderer UI, settlement, marketplace publishing, federation, or production playability. The current classification is `Runtime Boot Proven`, not `Playable`.
