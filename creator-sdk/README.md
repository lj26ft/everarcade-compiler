# Creator SDK Reality Notes

Pseudocode architecture:

- **What this module does:** provides template directories and a small Node CLI that can create, validate, build, deploy, and publish local JSON artifacts for creator projects.
- **Authority owner:** package/runtime authority is not owned here; the SDK only prepares non-authoritative creator metadata.
- **Non-authoritative:** generated `dist/*.json` files are local proof records, not production packages, live deployments, settlement events, marketplace listings, or playable hosted sessions.
- **Input consumed:** a template name, project path, `everarcade.game.json`, and optional CLI flags such as `--target` or `--channel`.
- **Output produced:** copied template files plus `dist/build.json`, `dist/package.json`, `dist/deployment.json`, and `dist/publication.json`.
- **Fit in EverArcade:** useful as a creator-facing prototype and audit fixture, but it does not yet bridge into the runtime appliance package format.


## World Package Certification vNext

The Creator SDK can now emit local certification artifacts that connect `world.evr` to certified kernel metadata, a signed world-package certificate, an independent proof re-check report, and deploy metadata. Run:

```bash
node creator-sdk/cli/everarcade.mjs certify-world --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs verify-world-certificate --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs deploy --project "$PROJECT"
```

Certification files are written to `dist/certification/`; deployment records the certificate only when it already exists. These artifacts are local certification records and do not change v0.1 architecture, runtime authority, canonicalizer behavior, or production readiness.
