# Technical Depth and Theme Report

## Contrast fixes

Added broad Docusaurus contrast hardening for generated documentation components, including badges, tags, pills, inline code, tables, admonitions, details, tabs, Mermaid text, buttons, cards, navbar, sidebar, TOC, and footer links.

## CSS files changed

- `src/css/custom.css`

## Pages added

- `docs/architecture/technical-architecture.md`
- `docs/operators/technical-operations.md`
- `docs/developers/technical-overview.md`
- `docs/developers/capabilities.md`
- `docs/concepts/why-everarcade-is-different.md`

## Architecture depth added

The architecture deep dive explains deterministic world runtime semantics, world packages, World Contracts, runtime packages, execution receipts, state roots, replay verification, checkpoints, federation, XRPL/Xahau anchoring boundaries, and Evernode deployment boundaries.

## Operator depth added

The operator deep dive explains what operators host, what they do not control, runtime package deployment, proof bundles, deployment flow, verification flow, and failure handling for crashes, checkpoints, divergent replay, failover, and archive recovery.

## Developer depth added

The developer overview explains deployable world contents, World Contract mutation rules, RustRigs status, runtime packages, asset/state packaging, local replay/debugging, and practical mutation examples.

## Capability matrix additions

The developer capability matrix maps core capabilities to maturity statuses aligned with `MATURITY.md`, including ALPHA, EXPERIMENTAL, SCAFFOLD, and PLANNED classifications where applicable. No capability is listed as PRODUCTION.

## Docusaurus validation result

`npm run docs:build` was run successfully after the documentation and CSS changes.

## Remaining recommendations

- Perform a browser visual inspection on the deployed site in both theme modes.
- Add automated visual regression coverage for generated Docusaurus components.
- Keep capability statuses synchronized with `MATURITY.md` as implementation evidence changes.
