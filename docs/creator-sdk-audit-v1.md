# Creator SDK Audit v1

## Objective

This audit reviews the current Creator SDK and world creation workflow from the perspective of a first-time game developer who starts with only:

```bash
git clone everarcade
```

The success metric is **Time To First World**: how quickly a developer can create, customize, run, project, package, verify, and understand deployment for a world without reading the architecture book.

Target: **under 30 minutes**. Stretch goal: **under 10 minutes**.

## Current shortest discovered path to Running World

The repository README provides the clearest current route. It requires the developer to run the Node CLI directly from the repo and know the `arena` template name:

```bash
TMPDIR="$(mktemp -d)"
PROJECT="$TMPDIR/arena-demo"
RUNTIME_ROOT="$TMPDIR/runtime-root"

node creator-sdk/cli/everarcade.mjs new --template arena --name arena-demo --dir "$PROJECT"
node creator-sdk/cli/everarcade.mjs build --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT"
```

Expected output:

```text
Playable Local Game: PASS
```

This is a working local path, but it is not yet the desired creator mental model:

```bash
everarcade world init --template frontier
everarcade world run
everarcade world package
everarcade world verify
```

## Stage 1: Repository Discovery

### First-time questions

| Question | Current answer | Friction |
| --- | --- | --- |
| Where do I start? | `README.md` quick start is the best entry. `docs/index.md`, `docs/developers/index.md`, and `docs/creator-sdk/quick-start.md` are additional starts. | F2 |
| What is a World Package? | Explained across world package docs and canonical package docs, while Creator SDK emits `dist/runtime-package`. | F3 |
| What is a World Contract? | Concept docs exist, but the first local template path does not require understanding the contract boundary. | F2 |
| What is a RustRig? | RustRigs docs define reusable domain modules, but there is no creator command that turns the concept into an action. | F3 |
| What template should I choose? | Templates exist under `creator-sdk/templates`, but discovery requires inspecting folders or reading scattered docs. | F3 |

### Measurements

- Minimum docs required for current Running World: **1** (`README.md`) if the developer finds the quick start immediately.
- Practical docs opened by a cautious first-time developer: **4-7** (`README.md`, Creator SDK quick start, developer index, concepts, RustRigs, world package docs, runtime bridge).
- Clicks or file opens before first command: **1-5** depending on whether the developer trusts the README.
- Concepts introduced before first success: **Creator SDK, template, build, test, runtime package, runtime root, local play, replay verification**.

## Stage 2: World Creation

### Current path

```bash
node creator-sdk/cli/everarcade.mjs new --template arena --name arena-demo --dir "$PROJECT"
```

### Desired path

```bash
everarcade world init --template frontier
```

### Extra steps and gaps

| Issue | Score | Severity | Notes |
| --- | --- | --- | --- |
| CLI is invoked as `node creator-sdk/cli/everarcade.mjs`, not `everarcade`. | F3 | High | This exposes repo internals immediately. |
| Command is `new`, not `world init`. | F2 | Medium | Generic verb does not reinforce world lifecycle. |
| Template name in desired path is `frontier`, but current obvious working template is `arena`. | F3 | High | Developer cannot map desired product language to current templates. |
| No `templates list` or `world templates` discovery command. | F3 | High | Requires file browsing. |
| `--dir`, `--project`, and positional name rules require explanation. | F2 | Medium | Minor but slows first run. |

## Stage 3: World Customization

A developer needs to find metadata, genesis, contract, continuity, and projection. Current discoverability is uneven:

| Area | Current location/signals | Friction |
| --- | --- | --- |
| Metadata | `everarcade.game.json`, generated `dist/runtime-package/world.json`, and package manifests. | F2 |
| Genesis | Present in some runtime examples and reports, not visible in the basic Creator SDK template mental model. | F3 |
| Contract | `guest_contract` manifest field or default contract code, plus `contracts/*` docs. | F3 |
| Continuity | Architecture/concept docs and runtime reports, not a first-template edit point. | F3 |
| Projection | Renderer/projection docs and projection demo examples, not a Creator SDK command. | F4 |

Confusing names and duplicates:

- `everarcade.game.json`, `world_manifest.toml`, `world.json`, `manifest.json`, and `package.json` all sound authoritative in different contexts.
- `creator-sdk/templates`, repo-level `templates`, `creator-marketplace/templates`, and `examples/template-worlds` compete for attention.
- `package` can mean Node package metadata, Creator SDK generated JSON, runtime package directory, world package, deployment package, or release package.

## Stage 4: RustRig Selection

Desired:

```bash
everarcade world add-rustrig combat
```

Current state:

- RustRig crates exist for domains such as combat, inventory, governance, movement, resources, market, crafting, factions, and continuity.
- Documentation explains RustRigs as reusable gameplay mutation libraries.
- There is no current Creator SDK command to list, add, certify, or explain RustRigs for a world project.

Gaps:

| Question | Current result | Score |
| --- | --- | --- |
| How do I add combat? | Read docs/crates manually; no CLI project mutation. | F4 |
| How do I add inventory? | Same. | F4 |
| How do I add governance? | Same. | F4 |
| How do I know what is certified? | Certification status is not surfaced in a creator command. | F3 |

## Stage 5: Projection

Desired:

```bash
everarcade world project
```

Current obstacles:

| Obstacle | Score |
| --- | --- |
| Projection is documented as a runtime/renderer concept rather than a creator lifecycle step. | F3 |
| No Creator SDK `project` command exists. | F4 |
| A first-time developer must distinguish authoritative runtime state from non-authoritative projection before launching anything. | F3 |
| Projection examples live outside the basic template path. | F3 |

## Stage 6: Packaging

Desired:

```bash
everarcade world package
```

Current:

```bash
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT"
```

Friction:

| Issue | Score |
| --- | --- |
| Package output is `dist/runtime-package`, not obviously `world.evr`. | F3 |
| The command works, but naming does not explain whether it is a world package, runtime package, or local placeholder package. | F3 |
| The developer may need to run `build` first, although the CLI auto-builds when needed. | F1 |

## Stage 7: Verification

Desired:

```bash
everarcade world verify
```

Current options:

```bash
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs certify-world --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs verify-world-certificate --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT"
```

Friction:

| Issue | Score |
| --- | --- |
| `test`, `certify-world`, `verify-world-certificate`, and replay verification overlap conceptually. | F3 |
| No single `world verify` command answers “is my package valid?” | F4 |
| Certificate language appears before the simple creator has a mental model for world packages. | F2 |

## Stage 8: Deployment

Desired:

```bash
everarcade world deploy
```

Current:

```bash
node creator-sdk/cli/everarcade.mjs deploy --project "$PROJECT" --target local
```

Deployment is currently local-proof-oriented. It writes deployment metadata and can include a certificate if already generated. Missing explanations:

- What `--target local` means.
- Difference between deploy, publish, Evernode deployment, release package, and live hosting.
- Which deployment paths are production, experimental, or scaffold.
- Whether a first-time developer should deploy before or after verification.

Friction score: **F3**.

## First-Time Developer Test Result

Persona: game developer, no EverArcade knowledge, no architecture knowledge.

| Task | Result | Score |
| --- | --- | --- |
| Discover templates | Possible by reading folders; no CLI discovery. | F3 |
| Discover RustRigs | Possible through docs/crates; no project-level command. | F3 |
| Understand continuity | Requires architecture/concept reading. | F3 |
| Build a world | Possible from README quick start. | F1 |
| Run a world | Possible, but command is verbose and exposes runtime root. | F2 |
| Package a world | Possible, but output naming is unclear. | F3 |
| Verify a world | Possible in pieces; not one obvious command. | F4 |
| Deploy a world | Local metadata deployment exists; live deployment expectations unclear. | F3 |

## Immediate conclusion

The current repo has enough pieces to reach a local playable proof, but the Creator SDK flow still asks the developer to learn internal vocabulary too early. The fastest improvement is a thin command and docs layer that presents a single world lifecycle and hides internals until requested.
