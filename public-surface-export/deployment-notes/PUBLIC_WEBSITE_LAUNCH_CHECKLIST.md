# Public Website Launch Checklist

## Messaging

- [x] Homepage leads with "Build Sovereign Worlds" rather than runtime internals.
- [x] Public definition: EverArcade is a sovereign game runtime for persistent worlds.
- [x] Calls to action include Get Started, Explore Documentation, View Architecture, and GitHub Repository.

## Navigation

- [x] Top navigation supports Home, Concepts, Developers, Operators, Players, Architecture, Documentation, Roadmap, and GitHub.
- [x] Role-specific landing pages are available without prior protocol knowledge.
- [x] Whitepaper is archived as historical reference.

## Broken Links

- [ ] Run `npm run docs:build` after dependency installation to perform final Docusaurus link validation.
- [x] Source-controlled sidebar points to canonical docs hierarchy.

## Documentation Coverage

- [x] Concepts section includes worlds, World Contracts, continuity, and RustRigs.
- [x] Developer path links Creator SDK, templates, runtime packages, World Contracts, and RustRigs.
- [x] Operator path links runtime operations, federation, proof bundles, and recovery procedures.
- [x] Architecture section preserves technical entry points.

## Mobile Experience

- [x] Docusaurus responsive layout selected.
- [ ] Validate on real mobile viewport during deployment preview.

## Contributor Experience

- [x] Contributor landing page links repository orientation and contributor guide.
- [x] Documentation framework configuration is source controlled.

## Developer Onboarding

- [x] Developer path reaches first-world creation resources within three clicks from home.
- [ ] Add richer screenshots or recorded walkthroughs after launch.

## Operator Onboarding

- [x] Operator page explains hosting, verification, replay proofs, and federation in plain language.
- [ ] Add operational diagrams from production deployment once available.

## Player Onboarding

- [x] Player page explains worlds, ownership, and continuity without runtime terminology.
- [ ] Add player-facing examples from live worlds after launch.

## Search Quality

- [x] Local-search plugin dependency and configuration are declared.
- [ ] Tune search synonyms and ranking after content analytics exist.

## Accessibility

- [x] Framework provides semantic navigation and keyboard-friendly defaults.
- [ ] Run Lighthouse and axe checks on deployment preview.

## Performance

- [x] Static generation framework selected.
- [ ] Run production build size and Lighthouse audits after dependency installation.
