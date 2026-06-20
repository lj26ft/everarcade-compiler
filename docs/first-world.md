# First World Quick Start

Create and verify your first EverArcade World in the shortest path.

```bash
everarcade world init --template frontier
cd everarcade-world

everarcade world run

everarcade world package

everarcade world verify
```

What you should see:

```text
World: PASS
Local Runtime Launch: PASS
World Package: PASS
WORLD VERIFY: PASS
```

Artifacts:

- `everarcade.game.json` describes your World.
- `dist/world.evr` is the creator-facing World Package entry point.
- `dist/runtime-package/` remains the compatibility directory used by existing runtime tooling.

Next: choose a template with `everarcade world templates`, inspect RustRigs with `everarcade world rustrigs`, then deploy locally with `everarcade world deploy`.
