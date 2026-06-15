# Developers

Build worlds, not just matches. EverArcade gives developers a path from a local idea to a portable world that operators can run and communities can verify.

## The journey

### 1. Create

Start with a world concept: rules, places, resources, player actions, and the history you want the world to remember.

- [Creator SDK](/docs/08-sdk-development)
- [Game developer start](/docs/GAME_DEVELOPER_START)
- [Reference world guide](/docs/reference-world-guide)

### 2. Deploy

Package the world so it can move from local development to hosted operation without becoming trapped in one interface.

- [Canonical package format](/docs/canonical-package-format)
- [Game package format](/docs/GAME_PACKAGE_FORMAT)
- [Evernode deployment](/docs/evernode-deployment)

### 3. Verify

Make world history checkable. Verification is how players, operators, and future maintainers can trust continuity without depending on memory or screenshots.

- [Replay verification](/docs/replay-verification)
- [Runtime release integrity](/docs/runtime-release-integrity)

### 4. Operate

Work with operators who host worlds, preserve recovery material, and help continuity survive upgrades and incidents.

- [Runtime operations manual](/docs/13-runtime-operations-manual)
- [Linux VM operator quickstart](/docs/linux-vm-operator-quickstart)

### 5. Scale

Use templates, reusable modules, and federation patterns as the world grows.

- [World runtime](/docs/05-world-runtime)
- [Federation runtime](/docs/06-federation-runtime)
- [Workload partitioning](/docs/workload-partitioning)

## Core building blocks

- **Creator SDK** — the developer entry point for authoring and packaging worlds.
- **World Contracts** — the shared rule layer that defines how a world changes.
- **RustRigs** — reusable gameplay modules and patterns for common world actions.
- **Documentation** — implementation details stay in `/docs`; the website stays focused on orientation.

## Next step

Read [Build your first world](/docs/GAME_DEVELOPER_START) or explore the [Founding Developers](/founding-developers) program.
