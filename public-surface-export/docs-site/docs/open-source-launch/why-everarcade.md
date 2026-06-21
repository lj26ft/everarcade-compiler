# Why EverArcade?

EverArcade exists because persistent digital worlds need stronger guarantees than traditional game-server ownership models usually provide.

## Traditional game servers

Most online games depend on private servers controlled by one studio or platform. Players and creators can experience a rich world, but the authoritative history is usually opaque. If the operator shuts down, changes rules, loses state, or refuses to provide evidence, the community has limited recourse.

## Platform ownership

Platform-owned worlds create a dependency mismatch: communities invest time, creativity, social trust, and economic meaning into worlds they do not control. EverArcade is built around portable artifacts and verifiable execution so the world can be understood independently from one hosting platform.

## Persistent worlds

A persistent world is more than a process. It is rules plus state plus history. EverArcade treats continuity as part of the world itself: journals, receipts, checkpoints, roots, replay reports, restore evidence, and migration evidence are not debug leftovers; they are the public memory of the world.

## Developer sovereignty

Developers should be able to define a world, compose known gameplay capabilities, package the result, and let others inspect the artifact. EverArcade uses World Contracts and World Packages to make the world definition portable instead of trapping it inside private deployment scripts.

## Self-custody

Self-custody in EverArcade means the important world artifact and evidence can be retained, inspected, and moved by the people responsible for the world. It does not require trusting a single marketplace, deployment operator, or opaque admin database as the only source of truth.

## Verifiable execution

Verifiable execution means a verifier can compare declared rules with emitted evidence. The verifier-facing path asks whether the World Contract, RustRig invariants, package hash, roots, receipts, replay output, restore output, and migration output agree.

## World continuity

Continuity is the promise that a world can remember. EverArcade's architecture exists so worlds can survive ordinary operational events: replaying history, restoring from checkpoints, upgrading packages, migrating state, and proving that the new state descends from the old one.
