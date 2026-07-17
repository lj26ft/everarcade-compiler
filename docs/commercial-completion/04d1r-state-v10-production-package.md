# Task 4D.1R — State V10 Production Package Authorization

This is a package-binding authorization, not evidence that State V10 has been implemented.

The compiler successor must bind State V10, Treasury v8, Treasury commitment v8, the 23-action inventory, the distinct failure and expiration action contracts, the receipt/action mapping, the amended state/event reachability rules, and the v7-to-v8 migration policy. Any change must alter Runtime IR and `world.evr` identity.

State V9 package bytes and verification behavior remain unchanged. A State V10 package must not activate through State V9 dispatch, and a State V9 package must never acquire successor semantics.

The only authorized semantic delta is:

```text
treasury.record_failure    + signed FAILED  -> RECEIVE_FAILED  -> FAILED
treasury.record_expiration + signed EXPIRED -> RECEIVE_EXPIRED -> EXPIRED
```

The implementation task must pin deterministic fixtures and mutation tests before production activation.
