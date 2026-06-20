# Creator SDK Friction Report

## Friction scale

- F0 = No friction
- F1 = Mild
- F2 = Moderate
- F3 = Significant
- F4 = Blocking

## Ranked issues

| Rank | Issue | Score | Severity | < 1 day fix candidate |
| --- | --- | --- | --- | --- |
| 1 | No single `everarcade world verify` command. | F4 | Critical | Add alias that runs package/certificate/replay checks or document current substitute. |
| 2 | No `everarcade world add-rustrig combat` path. | F4 | Critical | Add documented placeholder/alias that lists manual steps and certified status. |
| 3 | No `everarcade world project` path. | F4 | Critical | Add a projection placeholder command or README section linking one runnable demo. |
| 4 | CLI requires `node creator-sdk/cli/everarcade.mjs`. | F3 | High | Add npm bin, repo script, or shell wrapper named `everarcade`. |
| 5 | Template discovery requires folder inspection. | F3 | High | Add `everarcade templates` or `everarcade world init --list-templates`. |
| 6 | Desired `frontier` template does not map to obvious current template. | F3 | High | Add alias `frontier -> arena` or document current recommended template. |
| 7 | Package output is not `world.evr`. | F3 | High | Add a generated `world.evr` filename or explain `dist/runtime-package`. |
| 8 | Deployment terminology overlaps with publish, release, Evernode, and local metadata. | F3 | High | Add “which deploy command do I need?” docs. |
| 9 | Metadata/genesis/contract/continuity/projection are not presented in one creator map. | F3 | High | Add `docs/creator-sdk/world-project-map.md`. |
| 10 | Multiple package/manifest file names are confusing. | F2 | Medium | Add glossary table near quick start. |
| 11 | Current run path exposes `--runtime-root`. | F2 | Medium | Default runtime root and omit from quick start. |
| 12 | Creation command uses `new` instead of `world init`. | F2 | Medium | Add alias. |
| 13 | Build before package is conceptually extra, although auto-build exists. | F1 | Low | Document that package auto-builds. |

## Time To First World

Current **Time To First World** estimate:

- Experienced repo user: 10-20 minutes.
- First-time developer who follows `README.md` exactly: 20-45 minutes.
- First-time developer who detours into architecture docs: 60+ minutes.

Target **Time To First World**: under 30 minutes.
Stretch goal: under 10 minutes.
