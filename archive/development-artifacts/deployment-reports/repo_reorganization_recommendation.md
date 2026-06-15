# Repository Reorganization Recommendation

Audit date: 2026-05-30

This is a recommendation only. No files were moved in this audit pass.

## Proposed future layout

```text
crates/runtime/execution-core
crates/sdk/everarcade-sdk
crates/tools/studio-gui
crates/tools/tools
crates/cli/everarcade-cli
runtime/
templates/
examples/
deployment/
scripts/
docs/
```

## Goals

- Separate production-candidate deterministic runtime crates from scaffold/runtime-roadmap domains.
- Group SDK crates under a stable SDK namespace before external developer adoption.
- Keep creator tools and studio GUI together but separate from runtime authority.
- Make CLI ownership explicit instead of relying on `src-bin-everarcade` as the visible crate path.
- Preserve existing `runtime/`, `templates/`, `examples/`, `deployment/`, `scripts/`, and `docs/` top-level domains for non-crate assets.

## What should move later

### `execution-core` -> `crates/runtime/execution-core`

Move after fresh-VM validation is stable. This crate is the deterministic authority surface and should be grouped with runtime crates while preserving package name and semantics.

### `sdk/everarcade-sdk` -> `crates/sdk/everarcade-sdk`

Move only after SDK public API boundaries and versioning are documented. Other SDK crates can follow in the same namespace if they remain part of the supported developer surface.

### `studio-gui` -> `crates/tools/studio-gui`

Move when GUI workflow tests are stable. Studio should stay near tooling because it is a creator interface, not runtime authority.

### `tools` -> `crates/tools/tools`

Move after creator workflow responsibilities are split or documented. It currently mixes certification, protocol readiness, and creator pipeline helpers.

### `src-bin-everarcade` -> `crates/cli/everarcade-cli`

Move when CLI release naming is settled. This makes CLI ownership and packaging clearer.

## What should remain top-level

- `runtime/` — runtime assets, clients, renderer/client scaffold domains, deployment/runtime data, and non-authority runtime support.
- `templates/` — starter projects and deterministic creator templates.
- `examples/` — optional examples that should not block fresh-VM core validation until explicitly promoted.
- `deployment/` — manifests, reports, launch/deployment planning, and generated validation outputs.
- `scripts/` — validation and operational entrypoints; later split shared helpers into `scripts/lib/`.
- `docs/` — human-authored architecture, SDK, release, runtime, security, and XRPL documentation.

## Risks in the current layout

- Workspace crates live at mixed depths and naming conventions (`execution-core`, `studio-gui`, `src-bin-everarcade`, `sdk/*`, `runtime/*`).
- Reports are numerous and lack a single index distinguishing generated validation evidence from planning documents.
- Scripts are numerous and duplicate flag/job/vendor handling.
- Renderer/history/federation surfaces can be mistaken for production runtime authority unless explicitly documented as scaffold-level.
- Examples and templates are not clearly separated by release-critical vs optional status.

## Recommended migration sequence

1. Stabilize fresh-VM gates with current paths.
2. Add a repository report index and validation harness.
3. Move crates in small batches with compatibility redirects or clear release notes.
4. Update CI scripts and docs after each move.
5. Only then consider promoting scaffold surfaces to production-candidate status.

## Non-goals for this pass

- No directory moves.
- No crate renames.
- No broad script rewrites.
- No runtime semantics changes.
- No deletion of scaffold surfaces.
