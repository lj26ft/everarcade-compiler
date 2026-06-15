# Developer Flow Simplification

## Target flow

```text
everarcade new-game
everarcade add-rustrig combat
everarcade run
everarcade package
everarcade deploy
```

## Current command posture

The CLI currently exposes a broad internal surface: install/list/inspect/run/start game commands, asset commands, developer commands, replay and checkpoint tools, creator tools, runtime status commands, and many domain-specific runtime inspection commands. That is useful for validation but too large for first-time game developers.

## Recommended public grouping

| Public command | Should compose | User-facing purpose |
| --- | --- | --- |
| `everarcade new-game <id>` | Template copy, manifest initialization, deterministic defaults, first package skeleton. | Start a game with replay/checkpoint defaults already correct. |
| `everarcade add-rustrig <name>` | Rustrig manifest edit, package hash update, compatibility check. | Add gameplay capability without exposing package internals. |
| `everarcade run` | Local runtime start, replay directory setup, web/reference client launch hints, validation preflight. | Run deterministic local gameplay. |
| `everarcade package` | Manifest validation, package hash, rustrig hash, reproducibility metadata, receipt creation. | Produce deployable content. |
| `everarcade deploy` | Package verification, lease selection, EverNode/control-plane handoff, health gate. | Deploy through the operator control plane. |

## Commands to hide behind advanced mode

These should remain available for maintainers and CI but be hidden from the default help output:

- `install-game`, `list-games`, `inspect-game`, `run-game`, `start-game`, `init-game`, `build-game`, `package-game` because they are lower-level variants of `new-game`, `run`, and `package`.
- `run-dev`, `validate-game`, `deploy-game`, and `checkpoint-restore` because they should be grouped into `run`, `package`, `deploy`, and `recover` workflows.
- `replay-inspect`, `replay-verify`, `replay-diff`, and `multiplayer-sim` because they are debugging tools.
- `asset-register`, `asset-build`, `asset-verify`, `import-assets`, `package-content`, `validate-content`, and `publish-package` because they should be called by package/publish workflows.
- `runtime-*` status commands because they are operator/automation diagnostics, not first-run developer commands.

## Commands to defer

- Federation, renderer/history, and advanced observer commands should remain scaffold-level and stay out of the simple flow.
- Direct EverNode lease manipulation should remain operator-only until real EverNode API/process integration exists.
- Direct XRPL/Xahau publication commands should be deferred to an external settlement service.

## Recommended next bounded refactor

Implement aliases first, not a broad rewrite:

- `run` -> current local deterministic run path;
- `package` -> current package validation/hash path;
- `deploy` -> control-plane deployment handoff;
- `add-rustrig` -> manifest/package metadata update only.

Then reduce default help to the five-command happy path with an `everarcade advanced` or `everarcade doctor` namespace for existing lower-level tools.
