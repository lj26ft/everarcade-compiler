# Canonical world.evr Compiler

`everarcade-compiler` is the open conformance reference for turning a world configuration into a portable `world.evr` package. The open compiler owns profile resolution, PTW primitive composition, canonical package generation, conformance checks, package hashes, replay verification metadata, and SDK/CLI surfaces.

The compiler intentionally does not implement billing, marketplace operation, identity services, hosted registry operation, treasury execution, federation operation, AI platform services, client gateways, or commercial deployment orchestration. Those systems consume the package contract produced by this repository.

## Pipeline

```text
Canonical request -> profile graph -> contribution loading -> contribution graph -> merge -> Runtime IR -> Canonical Encoding -> world.evr Package -> Conformance -> Verification
```

## Reference CLI

```sh
world-evr create --config examples/world-factory/catacombs/world-request.json --dir /tmp/catacombs --out /tmp/catacombs.evr
world-evr verify --package /tmp/catacombs.evr
world-evr conformance --package /tmp/catacombs.evr
world-evr replay --package /tmp/catacombs.evr
world-evr diff --left /tmp/catacombs.evr --right /tmp/village.evr
```

The `everarcade world ...` command exposes the same implementation for compatibility.

## Package Contract

Generated packages include the canonical directories `manifest/`, `runtime/`, `world/`, `rules/`, `primitives/`, `content/`, `proof/`, `trust/`, `signatures/`, `assets/`, `projections/`, and `metadata/`. Determinism is enforced by fixed timestamps, sorted archive entries, deterministic JSON serialization, SHA-256 package hashes, and no random UUID generation.

## Integration Contract

```text
EverArcade Operator -> EverArcade Platform -> everarcade-compiler -> world.evr -> Registry -> Authority
```

The platform may call `compile`, `verify`, `loadProfiles`, `generatePackage`, and `validateConformance` equivalents, but the compiler remains infrastructure-independent.


## Runtime Assembly Engine profile resolution

Phase B adds a deterministic `everarcade.profile.v1` catalog and resolver. The compiler now resolves exact requested profiles, dependencies, optional-dependency decisions, conflicts, compiler capabilities, contribution namespaces, source provenance, and the `everarcade.profile-graph.v1` graph hash before contribution loading. Contribution graph construction and merge remain deferred to Phase C. See [`runtime-assembly-profile-resolution-v1.md`](runtime-assembly-profile-resolution-v1.md).


## Runtime assembly pipeline

```text
request → profile graph → contribution graph → deterministic merge → reference resolution → Runtime IR
```
