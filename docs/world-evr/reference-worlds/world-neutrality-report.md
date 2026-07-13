# Runtime Assembly World Neutrality Report

Catacombs and Simple Village share the same request adapter, profile resolver, contribution loader, merge engine, symbol/reference resolver, Runtime IR builder, package lowering, and hash functions. They also share the PTW runtime contract version, Runtime IR schema version, assembly-engine version, and package format.

They differ only by pinned declarative profiles and typed world content. Catacombs enables combat, spawning, encounters, and progression for an ARPG dungeon baseline. Simple Village enables only peaceful identity, movement, inventory, items, and interactions while explicitly serializing combat, spawning, encounters, and progression as disabled primitives.

Neutrality is guarded by tests that compare cross-world hashes, verify shared versions, verify projection-only Runtime IR hash stability, and scan generic assembly paths for reference-world-only identifiers outside allowed fixtures, built-in reference profile declarations, tests, and documentation.

## Current cross-world evidence

Simple Village and Catacombs produce different profile graph, contribution graph, merged contribution, reference graph, Runtime IR, runtime bundle, world contract, and package hashes while retaining the same Runtime IR schema and PTW runtime contract. The `world_neutral_cross_world_difference_and_shared_versions` test captures this contract.

## Catacombs regression note

The Catacombs runtime bundle and world contract hashes remain unchanged from the prior baseline. The Catacombs Runtime IR and package hashes drift in this phase because generic primitive serialization now honors an explicit `enabled: false` field when a primitive declaration is present; this is required so Simple Village can serialize disabled combat, spawning, encounter, and progression primitives explicitly without enabling their handlers. The drift is caused by generic Runtime IR serialization semantics, not by a Catacombs-specific branch.
