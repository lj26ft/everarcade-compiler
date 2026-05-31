# Marketplace Security Report

## Security guarantees
Marketplace packages are rejected when they request network access, filesystem access, authority writes, deployment execution, or XRPL submission.

## Runtime authority
Authority remains inside execution-core and authoritative runtimes. Marketplace packages can only consume records, emit records, compose records, validate records, and create deterministic behavior.

## Known limitations
Static flags model package authority requests in this scaffold-level validation harness.
