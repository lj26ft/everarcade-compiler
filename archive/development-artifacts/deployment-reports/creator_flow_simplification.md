# Creator Flow Simplification

## Target creator flow

```text
Create Project
Choose Template
Add Rustrigs
Build World
Run Locally
Publish
Deploy
```

## Classification of current GUI and creator surfaces

| Surface | Classification | Rationale | Recommendation |
| --- | --- | --- | --- |
| Project/workspace/session modules | scaffold | Project concepts exist, but they should be hardened around the same manifest/package path as the CLI. | Keep as the root of the Studio flow and hide raw filesystem choices. |
| Template selection | scaffold | Templates exist under `templates/` and creator examples exist, but GUI selection should enforce deterministic defaults. | Promote only known-good templates to the first screen. |
| Rustrig composer | scaffold | The surface exists and maps to creator needs, but live safety depends on package hash and ABI validation. | Let creators add approved rustrigs; keep custom rustrig authoring as advanced. |
| World builder, hierarchy, inspector, component editor | usable | These are central creator concepts and can be exposed if they write deterministic manifests/records. | Keep visible, but label unsaved edits as drafts until packaged. |
| Asset browser/import/catalog | usable | Asset import and validation are expected creator tasks. | Keep visible; require deterministic asset manifest validation before publish. |
| Local runtime/playmode/simulation | scaffold | Useful for local iteration, but must be tied to replay/checkpoint outputs. | Expose as `Run Locally`; surface replay root and deterministic validation status. |
| Replay UI/debugging/timeline | usable for diagnostics | Replay views help creators understand deterministic outcomes but can overwhelm first-run users. | Put under a Diagnostics tab, not the primary happy path. |
| Publishing/package preview/signing | scaffold | Package and publish concepts exist, but live signing/settlement must stay outside Studio authority. | Expose `Publish` as package creation plus control-plane handoff; avoid wallet custody. |
| Deployment/federation/node views | defer | These are operator concerns and currently scaffold-level for live EverNode. | Hide from creators until lease-per-game automation is real. |
| Diagnostics/health panels | scaffold | Useful for support, but raw health details are operator-oriented. | Show simple green/yellow/red status to creators; keep detailed panels advanced. |
| Marketplace/community registry | placeholder | Product surface is valuable, but live registry and settlement policy are not part of this bounded launch. | Defer from launch-critical Studio flow. |
| GPU/viewport renderer | scaffold | Visual projection is important but non-authoritative. | Present as preview only; never imply renderer output is source of truth. |

## Simplified Studio information architecture

1. **Create Project**: choose name, id, template, deterministic defaults.
2. **Choose Template**: curated templates only; advanced import behind a secondary action.
3. **Add Rustrigs**: approved rustrig list, compatibility status, package-hash preview.
4. **Build World**: world builder, hierarchy, inspector, assets.
5. **Run Locally**: local runtime, replay root, checkpoint status, deterministic validation result.
6. **Publish**: package validation, reproducible package hash, receipt preview.
7. **Deploy**: hand off package to control-plane/operator automation; show lease/health state only.

## Launch recommendation

Ship the Studio as a guided shell over proven CLI/package/runtime paths. Treat marketplace, federation views, live settlement, renderer/history expansion, and operator deployment controls as deferred or advanced surfaces until they are backed by deterministic records and live process integrations.
