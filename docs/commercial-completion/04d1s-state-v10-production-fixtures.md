# Task 4D.1S — State V10 Production Fixtures

The production fixture builder emits the exact locked State V10 tuple and exact 23-action inventory. Fixture parameters bind distinct world, Treasury, package, runtime, actor-key, governance, issuer, checkpoint, and workload identities.

Compiler validation rejects an incomplete or altered State V10 action inventory. Deterministic builds retain the Task 4D.1R baseline:

```text
Runtime IR: sha256:1cbfe2e08ea0e724076b38e5ddb5cf21b9f5b998f6ffc04c3260c6abcd230111
world.evr:  sha256:8f859c32508c864a7129845297f8cf91559f40ed43710e34bde89a14ddde8348
```

Fixture cloning starts from a verified checkpoint and content-object set. It does not mutate indexes or semantic state directly. This supports deterministic state/event and suffix comparisons while preserving the real Authority pathway.
