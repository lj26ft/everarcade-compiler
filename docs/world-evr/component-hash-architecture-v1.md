# World EVR Component Hash Architecture v1

Phase H packages expose canonical component hashes with distinct domain separators (`everarcade.hash.<component>.v1`). Runtime Hash covers authoritative execution declarations only: runtime metadata, limits, topology, regions, initial state, archetypes, entities, variables, primitives, actions, transitions, invariants, encounters, progression, authoritative content, checkpoint, journal, and root policy. Projection metadata is non-authoritative and excluded from Runtime Hash.

World Hash covers world identity, Runtime Hash, proof policy, economy metadata, runtime contract version, and primitive/ruleset metadata. Hosted deployment data is excluded.

Package Hash covers the canonical package file manifest and may change for projection-only edits while Runtime Hash and World Hash remain stable. `proof/hash-coverage-manifest.json` records component entries, domains, inclusion flags, source files, and authoritative status.
