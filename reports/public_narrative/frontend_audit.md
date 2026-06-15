# Frontend Audit

## Existing website stack

EverArcade already uses a Docusaurus site at the repository root. The current public website assets are source files in `src/pages`, `src/css/custom.css`, `docusaurus.config.js`, `sidebars.js`, and the Markdown documentation under `docs/`.

## Existing landing pages

Reusable source pages found during audit:

- `src/pages/index.js` — homepage and primary public landing page.
- `src/pages/developers.md` — developer journey page.
- `src/pages/operators.md` — operator journey page.
- `src/pages/players.md` — player journey page.
- `src/pages/concepts.md` — existing concept entry point.
- `src/pages/architecture.md` — technical architecture entry point.
- `src/pages/contributors.md` — contributor orientation page.

## Existing navigation and layout

The site uses Docusaurus navbar and footer configuration in `docusaurus.config.js`. The milestone should extend that navigation instead of creating a second website or a parallel application shell.

## Reusable branding and components

Reusable assets and patterns:

- Docusaurus `Layout` and `Link` components in the homepage.
- Existing hero, card grid, path grid, and section styles in `src/css/custom.css`.
- Existing color variables for light and dark themes.
- Mermaid support configured through `@docusaurus/theme-mermaid` for lightweight diagrams.
- Documentation under `docs/` for implementation references.

## Obsolete or lower-priority assets

No source assets need deletion for this milestone. The previous engineering-first entry points should remain available where useful, but top-level navigation should prioritize worlds, continuity, developers, operators, players, docs, roadmap, GitHub, and community.

Generated and dependency assets must remain uncommitted, including `node_modules/`, build output, `.docusaurus/`, coverage, screenshots, and generated indexes.

## Recommended structure

- Homepage: public narrative and role routing.
- `/worlds`: non-technical explanation of worlds.
- `/continuity-engine`: living world history narrative.
- `/developers`: create, deploy, verify, operate, scale.
- `/operators`: steward role, hosting, verification, federation.
- `/players`: persistence, ownership, history, continuity.
- `/founding-developers`: early builder recruitment.
- `/roadmap`: milestone progression without dates.
- `/community`: ecosystem relationships and lightweight Mermaid diagram.
- `/docs`: implementation and reference documentation.
