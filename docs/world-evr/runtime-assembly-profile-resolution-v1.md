# Runtime Assembly Profile Resolution v1

`everarcade.profile.v1` is the Phase B schema for deterministic Runtime Assembly Engine profile resolution. It resolves requested profile references into an exact, pinned `ResolvedProfileGraphV1` before contribution loading. Contribution payload loading and merge are deferred to Phase C.

## Identity and schema

A profile identity is `provider.category.name@version`, for example `everarcade.world.base-world-v1@1.0.0`. Display names, paths, catalog insertion order, request order, and mutable URLs are not part of identity. Each profile stores a separate content hash that is included in the graph hash.

Every `ProfileDefinition` declares provider namespace, name, exact version, schema version, content hash, category, dependencies, optional dependencies, conflicts, compiler capabilities, contribution namespaces, source provenance, deprecation metadata, and replacement metadata.

## Categories and namespaces

Supported categories are `world`, `runtime`, `genre`, `topology`, `biome`, `encounter`, `progression`, `economy`, `proof`, `projection`, `ai`, `access`, and `module`.

Supported contribution namespaces are `runtime`, `limits`, `topology`, `regions`, `spawn_points`, `archetypes`, `entities`, `primitives`, `actions`, `transitions`, `invariants`, `encounters`, `progression`, `content`, `projection`, `proof`, `economy`, `ai`, and `access`. Phase B validates declarations but does not merge payloads.

## Catalog and provenance

`ProfileCatalog` registers immutable definitions, performs exact lookup, explicit compatible lookup, deterministic enumeration, source provenance return, and catalog hashing. Initial source kinds are builtin, embedded fixture, local static profile, platform-supplied pinned bundle, and registered module bundle. Network fetching, latest-version lookup, and unpinned marketplace resolution are intentionally excluded.

## Dependency and version policy

Resolution canonicalizes root references, sorts them by identity, resolves exact versions, walks required dependencies, records optional dependency decisions, rejects missing required dependencies, rejects ambiguous compatible references, detects cycles, detects declared conflicts, validates compiler capabilities, validates contribution namespace declarations, and emits deterministic diagnostics.

Version ranges are only acceptable when an explicit compatible lookup produces one unambiguous catalog entry. The selected exact version and content hash are pinned in the resolved graph. The resolver never silently selects the newest version.

## Graph ordering and hash

The stable topological order is dependency-first and tie-broken by canonical identity. The graph hash uses the domain `everarcade.profile-graph.v1` and covers exact profile identities, versions, content hashes, dependency edges, optional selections, required capabilities, and contribution namespaces. Request order, catalog insertion order, filesystem paths, and local source paths do not affect the graph hash.

## Diagnostics

Profile-resolution diagnostics are structured with deterministic code, severity, assembly stage, profile identity, source provenance, message, and remediation. Phase B defines codes for not found, version not found, version ambiguity, dependency missing, dependency cycle, profile conflict, content hash mismatch, unsupported schema, invalid category, unsupported capability, and undeclared contribution namespace.

## Assembly pipeline

```text
Canonical request
→ profile graph
→ contribution loading
→ contribution graph
→ merge
→ Runtime IR
```

Phase B produces the profile graph and prepares the contribution-loading boundary. Contribution graph construction and deterministic merge are deferred to Phase C.
