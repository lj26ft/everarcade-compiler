# Strict PTW Conformance v1

Conformance is evaluated in ten mandatory layers: package structure, canonical encoding, manifest integrity, runtime contract, reference integrity, semantic coherence, bound completeness, invariant compatibility, determinism, and proof readiness. Results use `PASS`, `FAIL`, `INCONCLUSIVE`, or `UNSUPPORTED`; a skipped required layer cannot report `PASS`.

Diagnostics are typed for automated repair and include code, severity, layer, component, expected and actual values, source pointers when available, and remediation.
