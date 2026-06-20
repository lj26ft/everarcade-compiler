# Developer Experience Validation v1

## Purpose

This validation proves that a developer outside the core EverArcade team can create, customize, package, verify, project, and deploy a world by following repository artifacts instead of reading the architecture book.

## Scope

The validation uses `examples/world-creation-flow/frontier-validation/` as the canonical first-world exercise and `docs/canonical-world-creation-flow-v1.md` as the onboarding path.

## Success question

Can someone other than the core team build a world?

## Pass criteria

A developer can answer the following without core-team intervention:

- How do I create a world?
- How do I customize it?
- How do I package it?
- How do I verify it?
- How do I project it?
- How do I deploy it?

## Required evidence

The validation report must capture:

- world created
- package built
- verification passed
- projection launched or represented by projection configuration
- registry entry generated

## Thirty-minute target

A new developer should be able to complete this path within 30 minutes:

```bash
git clone <repo>
cd everarcade-compiler
# create or copy world template
# customize metadata, contract, genesis, continuity, projection
bash examples/world-creation-flow/frontier-validation/operator/build-world-evr.sh
bash examples/world-creation-flow/frontier-validation/operator/verify.sh
```

Future CLI equivalent:

```bash
everarcade world init --template frontier
everarcade world package
everarcade world verify
everarcade world project
```

## Validation commands

```bash
find reports/developer-experience-validation -type f | sort
grep -R "world init" docs
grep -R "world package" docs
grep -R "world deploy" docs
git diff --check
```
