# Public Narrative Launch Report

## Frontend audit

The existing public website is a Docusaurus site rooted in the repository. The milestone extended the existing source files rather than creating a second website. Reused assets include the homepage `Layout`/`Link` pattern, root Docusaurus navigation, shared CSS variables, card grid styles, and Mermaid support.

See `reports/public_narrative/frontend_audit.md` for the full audit.

## Pages created

- `/worlds` — world-first explanation of persistent places, communities, economies, history, and ownership.
- `/continuity-engine` — continuity translated into living world history.
- `/founding-developers` — early builder recruitment page with placeholder GitHub CTA.
- `/roadmap` — milestone progression without dates.
- `/community` — ecosystem relationship page using a lightweight Mermaid diagram.

## Pages updated

- Homepage — updated to “Build Worlds. Not Just Games.” with CTA hierarchy: Explore Worlds, Documentation, GitHub.
- `/developers` — reorganized around Create, Deploy, Verify, Operate, Scale.
- `/operators` — explains stewardship, hosting, verification, and federation in public language.
- `/players` — explains persistence, ownership, history, and continuity without architecture detail.

## Navigation changes

Top-level navigation now prioritizes:

- Home
- Worlds
- Developers
- Operators
- Players
- Continuity Engine
- Documentation
- Roadmap
- GitHub
- Community

## Narrative changes

The public narrative now moves from “Deterministic Runtime” toward “Build Sovereign Worlds.” Engineering-first terms are mapped to public language in `PUBLIC_NARRATIVE_AUDIT.md`, including:

- Deterministic Replay → Verifiable World History
- State Lineage → World Continuity
- Execution Equivalence → Independent Verification

## Documentation integration

The website now acts as vision and onboarding. Implementation details remain linked into existing `docs/` pages instead of being duplicated in public landing pages. Developer, operator, roadmap, and continuity pages all route readers into relevant documentation sources.

## Launch blockers

- `npm run docs:build` could not run because the `docusaurus` executable is not available in the current environment path.
- Untracked generated/dependency folders already exist in the working tree: `node_modules/` and `runtime/hotpocket-gameplay-proof/node_modules/`. They were not staged or committed.
- The exact large-file scan reports existing large dependency/generated/vendor files; no new large source files were introduced by this milestone.

## Recommended next steps

1. Install or expose Docusaurus dependencies in a clean environment and run the docs build.
2. Remove or ignore untracked dependency folders before release packaging.
3. Add real community and founding-developer participation links when the public channels are ready.
4. Continue replacing engineering-first language on deeper documentation pages as they are revised.
5. Add lightweight examples or screenshots only if they can remain source-controlled and below the size limits.

## Validation results

### `git diff --stat`

```text
STATUS
?? PUBLIC_NARRATIVE_AUDIT.md
?? TESTNET_LAUNCH_PAGE.md
?? node_modules/
?? reports/public_narrative/
?? runtime/gpu-marketplace/Cargo.lock
?? runtime/hotpocket-gameplay-proof/node_modules/
BIGFILES
./.git/index
./.git/objects/pack/pack-dfc879dc4d16dbfe99b864502336c60261011a77.pack
./node_modules/@rolldown/binding-linux-x64-gnu/rolldown-binding.linux-x64-gnu.node
./node_modules/lightningcss-linux-x64-gnu/lightningcss.linux-x64-gnu.node
./node_modules/react-dom/cjs/react-dom-client.development.js
./node_modules/react-dom/cjs/react-dom-client.production.js
./node_modules/react-dom/cjs/react-dom-profiling.development.js
./node_modules/react-dom/cjs/react-dom-profiling.profiling.js
./node_modules/typescript/lib/_tsc.js
./node_modules/typescript/lib/lib.dom.d.ts
./node_modules/typescript/lib/lib.webworker.d.ts
./node_modules/typescript/lib/typescript.d.ts
./node_modules/typescript/lib/typescript.js
./node_modules/vite/dist/node/chunks/node.js
./release/VENDOR_SHA256SUMS
./runtime/hotpocket-contract-proof/node_modules/libsodium/dist/modules-esm/libsodium.mjs
./runtime/hotpocket-contract-proof/node_modules/libsodium/dist/modules/libsodium.js
./runtime/hotpocket-gameplay-proof/node_modules/libsodium/dist/modules-esm/libsodium.mjs
./runtime/hotpocket-gameplay-proof/node_modules/libsodium/dist/modules/libsodium.js
```

Note: the captured validation file combined command sections; `git diff --stat` itself had no output after the first two commits because only untracked report files remained.

### `git status --short`

```text
?? PUBLIC_NARRATIVE_AUDIT.md
?? TESTNET_LAUNCH_PAGE.md
?? node_modules/
?? reports/public_narrative/
?? runtime/gpu-marketplace/Cargo.lock
?? runtime/hotpocket-gameplay-proof/node_modules/
```

### `find . -type f -size +500k | sort`

```text
./.git/index
./.git/objects/pack/pack-dfc879dc4d16dbfe99b864502336c60261011a77.pack
./node_modules/@rolldown/binding-linux-x64-gnu/rolldown-binding.linux-x64-gnu.node
./node_modules/lightningcss-linux-x64-gnu/lightningcss.linux-x64-gnu.node
./node_modules/react-dom/cjs/react-dom-client.development.js
./node_modules/react-dom/cjs/react-dom-client.production.js
./node_modules/react-dom/cjs/react-dom-profiling.development.js
./node_modules/react-dom/cjs/react-dom-profiling.profiling.js
./node_modules/typescript/lib/_tsc.js
./node_modules/typescript/lib/lib.dom.d.ts
./node_modules/typescript/lib/lib.webworker.d.ts
./node_modules/typescript/lib/typescript.d.ts
./node_modules/typescript/lib/typescript.js
./node_modules/vite/dist/node/chunks/node.js
./release/VENDOR_SHA256SUMS
./runtime/hotpocket-contract-proof/node_modules/libsodium/dist/modules-esm/libsodium.mjs
./runtime/hotpocket-contract-proof/node_modules/libsodium/dist/modules/libsodium.js
./runtime/hotpocket-gameplay-proof/node_modules/libsodium/dist/modules-esm/libsodium.mjs
./runtime/hotpocket-gameplay-proof/node_modules/libsodium/dist/modules/libsodium.js
```
