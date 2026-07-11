# Canonical world.evr Compiler

`everarcade-compiler` is the open conformance reference for turning a world configuration into a portable `world.evr` package. The open compiler owns profile resolution, PTW primitive composition, canonical package generation, conformance checks, package hashes, replay verification metadata, and SDK/CLI surfaces.

The compiler intentionally does not implement billing, marketplace operation, identity services, hosted registry operation, treasury execution, federation operation, AI platform services, client gateways, or commercial deployment orchestration. Those systems consume the package contract produced by this repository.

## Pipeline

```text
World Configuration -> Profile Resolution -> Primitive Composition -> Rule Composition -> Projection Metadata -> Economy Metadata -> Proof Metadata -> Canonical Encoding -> world.evr Package -> Conformance -> Verification
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
