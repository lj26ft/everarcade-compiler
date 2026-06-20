# World Project Map

This is the first map to read after creating a World. It explains what each creator-facing area is for and when to edit it.

| Area | Purpose | File location | Modification guidance |
| --- | --- | --- | --- |
| World Metadata | Names the World, selected template, runtime target, assets, features, and optional guest contract. | `everarcade.game.json` | Edit name, template-facing labels, assets, and feature metadata before packaging. Keep paths valid. |
| Genesis | Defines the starting state implied by the selected template and starter code. | `src/game.js` and template assets | Change starter entities, actions, and initial values in small steps; run `everarcade world verify` after changes. |
| World Contract | Contains the deterministic guest logic when a template uses a Rust contract. | `contracts/arena-proof-contract` or the `guest_contract` path in metadata | Rust developers may edit contract code, but should keep exported entry points stable. |
| Continuity | Records package identity, compatibility, certificates, and verification reports. | `dist/world.evr`, `dist/runtime-package/`, `dist/certification/` | Do not hand-edit generated files; regenerate with `everarcade world package` or `everarcade world verify`. |
| Projection | Makes the World visible through a local projection or demo surface. | `renderer/projection/README.md` and projection runtime docs | Treat as a discoverable local view, not authority. Use `everarcade world project` to find the entry point. |
| Registry | Lists package identity and proof registry metadata for certification. | `dist/certification/formal-proof-registry.json` | Generated during verification. Review when debugging certification only. |
| Proofs | Captures checks, hashes, replay evidence, and independent re-check results. | `dist/certification/independent-proof-recheck.json` and runtime proof reports | Generated evidence. Commit only when a report is intentionally part of an audit or release record. |
