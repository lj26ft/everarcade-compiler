# Runtime Assembly Contribution Merge v1

Phase C introduces `everarcade.contribution.v1`, `everarcade.contribution-graph.v1`, and `everarcade.merged-contributions.v1` between profile resolution and later reference resolution.

Pipeline:

```text
request → profile graph → contribution graph → deterministic merge → reference resolution → Runtime IR
```

## Identity and provenance

Every contribution has a stable identity made from its contribution id, namespace, declaration key, schema version, source profile id/version/content hash, and contribution content hash. It excludes filesystem paths, catalog insertion order, request order, and local absolute paths. Provenance records the profile identity, version, profile content hash, source kind, dependency path, namespace, contribution id, contribution content hash, and requested root profile that introduced the contribution.

## Typed namespaces

The contract has explicit typed payload variants for `runtime`, `limits`, `topology`, `regions`, `spawn_points`, `entity_archetypes`, `item_archetypes`, `encounter_archetypes`, `entities`, `world_variables`, `primitives`, `actions`, `transitions`, `invariants`, `encounters`, `progression`, `content`, `projection`, `proof`, `economy`, `ai`, and `access`. Payload wrappers carry an `authoritative` flag so authoritative declarations remain distinguishable from advisory or projection-oriented data.

## Contribution graph

`ContributionGraphV1` contains resolved profile nodes, contribution nodes, profile-to-contribution edges, contribution dependency edges, ordering constraints, override relationships, provenance, and a stable graph hash. The graph hash uses the `everarcade.contribution-graph.v1` domain and covers contribution identities, namespaces, declaration keys, source profile hashes, ordering, overrides, dependencies, and merge policies.

## Merge classes

* **Unique declaration**: one effective declaration may exist; non-identical duplicates fail with `ASSEMBLY_UNIQUE_DECLARATION_CONFLICT`.
* **Keyed union**: stable keys are inserted deterministically; byte-identical duplicates deduplicate; non-identical same-key declarations fail without an explicit authorized override.
* **Bounded aggregation**: named rules, such as `strictest_maximum`, combine values deterministically. Rules are explicit and not inferred from field types.
* **Ordered composition**: stable order keys and declared dependencies determine semantic order. Discovery order is never semantic order.
* **Explicit override**: overrides are valid only when the field permits override, the operation is `override`, the exact target and expected target content hash are declared, source authorization passes, and the override is recorded in provenance.

## Override policies

The model includes `forbidden`, `same_provider_only`, `declared_extension_point`, `request_override_allowed`, `operator_policy_required`, and `exact_target_hash_required`. Silent last-write-wins is forbidden.

## Merged output and hashes

`MergedContributionSetV1` stores typed effective collections, applied overrides, deduplicated identical contributions, aggregation decisions, ordering decisions, provenance index, diagnostics, and the merged contribution hash. The merged hash uses the `everarcade.merged-contributions.v1` domain and covers the effective declarations after deterministic merge decisions.

## Diagnostics

Phase C reserves structured diagnostics for duplicate, conflict, unique declaration conflict, invalid aggregation, ordering conflict/cycle, forbidden or unauthorized overrides, missing override targets, target hash mismatch, version incompatibility, namespace mismatch, and unsupported schemas. Diagnostics include stage, namespace, declaration key, contribution/source identifiers, deterministic messages, and remediation guidance.

## Deferred work

Symbolic world references such as entity-to-region links are intentionally not resolved in this phase. They are inputs to the later reference-resolution phase.
