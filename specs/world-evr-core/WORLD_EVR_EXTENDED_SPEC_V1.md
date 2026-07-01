# WORLD_EVR_EXTENDED_SPEC_V1

## Status

`WORLD_EVR_EXTENDED_SPEC_V1` defines optional `world.evr` namespaces for commercial, presentation, and auxiliary capabilities. Extended content may evolve independently and MUST NOT affect Core replay determinism or invalidate Core proofs.

## Optional namespaces

A world MAY include any, all, or none of these directories:

```text
world.evr/
├── assets/
├── projections/
├── ai/
└── metadata/
```

A world that omits every Extended directory remains a valid world when it satisfies `WORLD_EVR_CORE_SPEC_V1`.

## `assets/`

Purpose: content-addressed assets used by projections.

Examples: models, textures, audio, animations, UI resources.

Requirements:

- asset descriptors SHOULD live in `assets/assets.json`;
- every load-bearing asset reference MUST include a hash and version;
- assets MUST be content-addressed by digest or immutable URI;
- missing or failed assets MUST NOT invalidate Core replay.

## `projections/`

Purpose: visualizations of Core reality.

Examples: browser, native, mobile, and VR clients.

Requirements:

- projection descriptors SHOULD live in `projections/projections.json`;
- each projection SHOULD include `projection_id`, `version`, `runtime_requirements`, and hash references;
- projection certification is independent from Core certification;
- projection failure MUST NOT invalidate replay, restore, migration, or Core verification.

## `ai/`

Purpose: AI systems associated with the world.

Examples: NPC brains, dungeon masters, simulation agents.

Requirements:

- AI descriptors SHOULD live in `ai/ai.json`;
- descriptors SHOULD include policy metadata, permissions, versions, and hashes;
- AI execution artifacts MUST enter deterministic world execution only through Core-approved actions, receipts, or modules;
- AI systems MUST NOT bypass deterministic world execution.

## `metadata/`

Purpose: human-readable platform information.

Examples: screenshots, categories, genres, tags, documentation links, promotional descriptions.

Requirements:

- metadata descriptors SHOULD live in `metadata/metadata.json`;
- metadata MAY change without invalidating Core replay proofs;
- platform metadata certification, if present, MUST be reported separately from Core certification.

## Extended hash and certification

Extended content SHOULD be bound by an `extended_hash` computed from byte-lexic directory hashes for included Extended namespaces. The `extended_hash` is not an input to the Core `world_hash`. Commercial platforms MAY require extended certification for display, distribution, or marketplace features, but MUST label it separately from Core certification.

## Reserved future namespaces

The following names are reserved and are not part of Core v1 or Extended v1 unless a later specification activates them:

```text
federation/
economy/
identity/
social/
marketplace/
archives/
environment/
build/
history/
```

Core v1 verifiers MUST ignore but preserve these namespaces when encountered unless a future spec version explicitly marks them required.
