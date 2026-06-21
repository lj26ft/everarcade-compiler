# Creator SDK Roadmap v1

## Target experience

The target creator workflow is:

```bash
everarcade world init --template frontier
everarcade world add-rustrig combat
everarcade world run
everarcade world package
everarcade world verify
everarcade world deploy
```

Everything else should be optional until after the first successful world.

## Product principle

The architecture is not the product. The developer experience is the product. The Creator SDK should present one obvious world lifecycle and defer runtime internals, package internals, deployment internals, and certification details until the developer asks for them.

## Success metric

- Current metric: **Time To First World**.
- Target: **under 30 minutes**.
- Stretch goal: **under 10 minutes**.

## Roadmap

### Phase 1: Command aliases and docs

- Add `everarcade` executable path or documented npm script.
- Add `world init`, `world run`, `world package`, `world verify`, and `world deploy` aliases.
- Add template discovery.
- Add a single Creator SDK quick start that does not require architecture docs.

### Phase 2: Template clarity

- Define the recommended beginner template.
- Add `frontier` alias if product docs use that name.
- Explain blank, arena, rpg, trading, civilization, sandbox, and marketplace-demo templates in one table.
- Add template suitability: first game, multiplayer proof, economy, governance, persistence, projection.

### Phase 3: RustRig workflow

- Add `world rustrigs` to list available RustRigs and certification/maturity status.
- Add `world add-rustrig combat`, `inventory`, and `governance` as first supported examples.
- Make the command update project metadata and print next steps.

### Phase 4: Package and verify

- Make `world package` emit or clearly point to `world.evr`.
- Make `world verify` answer one question: “Can this package run and replay locally?”
- Keep certificate/proof details in verbose output or `--explain`.

### Phase 5: Projection and deployment

- Add `world project` for a local non-authoritative projection.
- Add `world deploy --target local` as the safe default.
- Clearly label Evernode/live deployment as experimental until release gates say otherwise.

## Non-goals

- No new runtime architecture.
- No production-readiness claims.
- No requirement that first-time creators understand federation, settlement, GPU marketplace, renderer internals, or HotPocket adapters before first success.
