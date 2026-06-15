# Documentation Portal Launch Report

## Implementation decisions

- Selected Docusaurus because it supports static generation, Markdown-first authoring, source-controlled sidebars, mobile responsive layouts, dark mode, versioning readiness, Mermaid diagrams, and contributor-friendly repository workflows.
- Configured the documentation portal at `/docs` so the site can be deployed as `everarcade.games/docs` or fronted by `docs.everarcade.games`.
- Kept the canonical `docs/` hierarchy as the content source instead of moving documentation into a separate website-only tree.
- Added local search configuration through `@easyops-cn/docusaurus-search-local` so search can be generated with the static build without depending on a hosted search provider at launch.

## Content migration and rewrite

- Reframed public onboarding from "read the whitepaper" to "choose your path."
- Added a world-first homepage that explains EverArcade as a sovereign game runtime for persistent worlds.
- Added role landing pages for developers, operators, players, and contributors.
- Added beginner-friendly concept pages for worlds, World Contracts, continuity, and RustRigs.
- Preserved architecture depth in a dedicated architecture section rather than making deterministic runtime internals the first reader experience.
- Retired the whitepaper into `/docs/archive/whitepaper` as historical reference.

## Navigation structure

Primary navigation now follows the target information architecture:

1. Home
2. Concepts
3. Developers
4. Operators
5. Players
6. Architecture
7. Documentation
8. Roadmap
9. GitHub

The sidebar starts with the documentation index, then guides readers through choose-your-path onboarding, concepts, developers, operators, architecture, and archive material.

## Visual architecture library

Added Mermaid-first diagrams for:

- System Overview
- Run → Replay → Verify → Restore → Continue
- World Lifecycle
- Stakeholder Model
- World Contract Architecture
- Federation Architecture

These diagrams are source controlled and reproducible by the documentation build.

## Launch readiness findings

- Messaging is ready for public onboarding and no longer requires protocol knowledge in the first minute.
- Developer onboarding reaches first-world resources within three clicks from the homepage.
- Operator onboarding introduces hosting, replay proofs, verification, federation, and recovery in plain language.
- Player onboarding explains persistence, ownership, and continuity without requiring runtime terminology.
- Remaining launch checks should be completed in a deployment preview: production build, full link validation, mobile viewport review, Lighthouse, axe accessibility checks, and search quality tuning.

## Recommended follow-up improvements

1. Add deployment-specific domain routing for `docs.everarcade.games` if the docs portal is hosted separately from the marketing homepage.
2. Add versioned docs once the first public release train is tagged.
3. Add screenshots or short walkthroughs for Create Your First World and operator recovery workflows.
4. Add real-world examples to the player pages after the first public worlds are available.
5. Tune local search stop words and synonyms after observing launch analytics.
6. Add CI validation for `npm run docs:build` once Docusaurus dependencies are installed in the build environment.
