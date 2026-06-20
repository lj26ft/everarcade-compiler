# Friction Audit

| Severity | Area | Finding | Mitigation |
| --- | --- | --- | --- |
| critical | missing CLI surfaces | `everarcade world init/package/verify/project/deploy` are not implemented yet. | Track the specification in `docs/world-cli-surface-v1.md`; use operator scripts as current equivalents. |
| high | missing automation | Packaging and verification require example-local shell scripts. | Promote scripts into a real CLI once command surface is approved. |
| high | confusing terminology | World Package, World Contract, RustRig, Projection Runtime, and Continuity Engine need first-run definitions. | Keep definitions in onboarding docs and validation checklist. |
| medium | missing examples | A Frontier-specific canonical first-world example did not exist before this validation. | Use `examples/world-creation-flow/frontier-validation/` as the canonical example. |
| medium | missing validation | Validation is currently file/package-level rather than full runtime-level. | Add CLI-backed schema and projection checks in the next milestone. |
| low | missing docs | Operator handoff was implicit for first-world developers. | Add a handoff file to the validation example. |
