# Native verifier artifact boundary

The native verifier consumes compiler-produced frozen artifacts; it does not produce Runtime IR or `world.evr`. P3 reuses the State V10 commitments and P2 formats byte-for-byte. Compiler package generation and historical package interpretation remain outside the Rust verifier.

Certification retains exact Runtime IR and `world.evr` equality through the final-freeze verifier. Native binary identity is performance evidence, not a protocol commitment.
