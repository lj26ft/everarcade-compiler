# P2 non-semantic acceleration artifacts

The compiler does not emit or commit the P2 authenticated directory or packed index. Runtime IR and `world.evr` remain frozen and byte-identical. Authority creates the companion artifacts from a verified compiler package and frozen checkpoint, and binds both package hashes into the directory.

Consequently P2 requires no compiler semantic change, new tuple, or fallback dispatch. Compiler deterministic-rebuild and final-freeze verification remain the artifact oracle.
