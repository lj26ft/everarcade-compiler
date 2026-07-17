# Task 4D.1R — State V10 Production Package

The compiler now binds the exact State V10/Treasury v8 tuple and rejects mismatched predecessor/successor tuples.

The compiler successor must bind State V10, Treasury v8, Treasury commitment v8, the 23-action inventory, the distinct failure and expiration action contracts, the receipt/action mapping, the amended state/event reachability rules, and the v7-to-v8 migration policy. Any change must alter Runtime IR and `world.evr` identity.

State V9 package bytes and verification behavior remain unchanged. A State V10 package must not activate through State V9 dispatch, and a State V9 package must never acquire successor semantics.

The only authorized semantic delta is:

```text
treasury.record_failure    + signed FAILED  -> RECEIVE_FAILED  -> FAILED
treasury.record_expiration + signed EXPIRED -> RECEIVE_EXPIRED -> EXPIRED
```

The retained focused fixture rebuilds deterministically. Its Runtime IR is `sha256:1cbfe2e08ea0e724076b38e5ddb5cf21b9f5b998f6ffc04c3260c6abcd230111`; `world.evr` is `sha256:8f859c32508c864a7129845297f8cf91559f40ed43710e34bde89a14ddde8348`. The world-bound Treasury and State genesis roots are `sha256:02adfdac30cb5bf5116b71635d14451971409bfea9ed7a4e22dd3981061967f4` and `sha256:d46eda7f1b42a5b8d72d0551751d9c7d198073b27eaee8e06f67f6f20dd1b054`.
