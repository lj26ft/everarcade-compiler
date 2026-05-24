# Federation Simulation Architecture

Federation simulation validates deterministic equivalence. It does not implement distributed consensus.

This architecture models sovereign runtime appliances as independently executable nodes that must converge on identical replay roots, receipt roots, state roots, and archive roots when fed equivalent deterministic workloads.

## Key semantics
- Federation philosophy: each appliance is sovereign, but validation requires reproducible outcomes.
- Replay synchronization: replay verification order is deterministic and fixture-defined.
- Archive synchronization: archives are validated by deterministic hash continuity and reconstruction checks.
- Topology responsibilities: topology manifests define expected memberships and peer edges only.
- Divergence expectations: controlled fault injection validates detection and recovery diagnostics.
- Restoration model: a restored node must rejoin with replay root equivalence to federation canonical root.
- Evernode assumptions: deployment assumes static membership epochs, deterministic build/runtime, and artifact continuity.
