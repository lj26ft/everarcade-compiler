# Runtime Namespace Governance

This document defines deterministic runtime namespace governance:
- explicit module ownership for replay/history, transport, federation, validation, CI, and proof materialization runtimes;
- explicit import discipline (no wildcard export dependency in runtime governance surfaces);
- continuity-first replay namespace lineage to preserve reconstruction-only semantics.
