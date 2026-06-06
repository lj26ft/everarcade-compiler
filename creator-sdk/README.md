# Creator SDK Reality Notes

Pseudocode architecture:

- **What this module does:** provides template directories and a small Node CLI that can create, validate, build, deploy, and publish local JSON artifacts for creator projects.
- **Authority owner:** package/runtime authority is not owned here; the SDK only prepares non-authoritative creator metadata.
- **Non-authoritative:** generated `dist/*.json` files are local proof records, not production packages, live deployments, settlement events, marketplace listings, or playable hosted sessions.
- **Input consumed:** a template name, project path, `everarcade.game.json`, and optional CLI flags such as `--target` or `--channel`.
- **Output produced:** copied template files plus `dist/build.json`, `dist/package.json`, `dist/deployment.json`, and `dist/publication.json`.
- **Fit in EverArcade:** useful as a creator-facing prototype and audit fixture, but it does not yet bridge into the runtime appliance package format.
