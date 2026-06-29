# Review Request

## Requested Review Areas

### Commitment Architecture

Questions:

- Are roots binding?
- Is domain separation complete?
- Are commitment profiles sufficient?

### Receipt MMR

Questions:

- Can inclusion proofs be forged?
- Can second-preimage scenarios exist?
- Is peak ordering unambiguous?
- Is empty root handling safe?

### Checkpoints

Questions:

- Is chaining sound?
- Can checkpoints be replayed or reordered?
- Are external anchors sufficiently specified?

### Proof Format

Questions:

- Are proofs self-contained?
- Can malformed proofs bypass verification?
- Are rejection rules complete?

### Conformance Vectors

Questions:

- Are adversarial cases sufficient?
- Are negative vectors comprehensive?
- Are additional attack cases needed?

### Benchmarks

Questions:

- Are synthetic assumptions reasonable?
- What should be validated first on hardware?
- What likely becomes the next bottleneck?
