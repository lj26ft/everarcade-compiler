# Independent Verification

The RC2 release of `everarcade-compiler` was independently reviewed and reproduced
from a clean public clone by **Dane Brown — Kairo Vault**
(https://kairovault.com), acting as independent reviewer of record.

The reviewer acted independently and is not an employee, officer,
or affiliate of EverArcade.

## Verified

- Trust-chain fixes — the RC2 trust chain was reviewed and issues were surfaced and corrected.
- Deterministic build and verification gate — reproduced from a clean public clone.
- Negative fixtures — all intended failure cases were correctly rejected.
- Commit-pin checker — verified cold, including fallback execution paths.

## Reproduce It Yourself

Do not trust this statement alone.

Re-run the process yourself:

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
scripts/ci/check-rc2-commit-pins.sh
# then run the build / verification gate per the repository instructions
```

A clean clone, the same gate, the same result.

## Scope

This was an independent reproducibility review and is not a comprehensive security audit.

No warranty is implied.

The review attests that the RC2 release builds, verifies, and reproduces from the public source as described. It does not imply that the codebase is free from all defects, vulnerabilities, or future regressions.

---

Reviewer:
Dane Brown, Kairo Vault

Reviewed RC2 as of commit:
b9f12ac
