# Creator SDK

Pseudocode architecture:

- **What this module does:** provides template directories and a small Node CLI that can create, run, package, verify, deploy, and publish local World artifacts for creator projects.
- **Authority owner:** package/runtime authority is not owned here; the SDK only prepares non-authoritative creator metadata.
- **Non-authoritative:** generated `dist/*.json` files and `dist/world.evr` are local proof records, not production packages, live deployments, settlement events, marketplace listings, or playable hosted sessions.
- **Input consumed:** a template name, project path, `everarcade.game.json`, and optional CLI flags such as `--target` or `--channel`.
- **Output produced:** copied template files plus `dist/build.json`, `dist/world.evr`, `dist/deployment.json`, and `dist/publication.json`.
- **Fit in EverArcade:** useful as a creator-facing prototype and audit fixture, but it does not yet bridge into the runtime appliance package format.

## First World flow

```bash
everarcade world init --template frontier
cd everarcade-world

everarcade world templates
everarcade world rustrigs
everarcade world run
everarcade world package
everarcade world verify
everarcade world deploy
```

Use `node creator-sdk/cli/everarcade.mjs world ...` from a source checkout when the `everarcade` package binary has not been linked.


## World Package Certification vNext

The Creator SDK can now emit local certification artifacts that connect `world.evr` to certified kernel metadata, a signed world-package certificate, an independent proof re-check report, and deploy metadata. Run:

```bash
everarcade world verify --project "$PROJECT"
everarcade world deploy --project "$PROJECT"
```

Certification files are written to `dist/certification/`; deployment records the certificate only when it already exists. These artifacts are local certification records and do not change v0.1 architecture, runtime authority, canonicalizer behavior, or production readiness.
