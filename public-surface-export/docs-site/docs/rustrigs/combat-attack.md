# Certified RustRig: `combat.attack()`

`combat.attack()` is the canonical reference RustRig for deterministic combat state mutation in EverArcade worlds. It complements `inventory.transfer()` by proving deterministic gameplay state transitions for health, damage, death, receipts, replay, and root verification.

## Mutation Declaration

World Contracts declare the certified mutation surface with the `everarcade-rustrig-combat` crate:

```toml
[rustrigs.combat.attack]
crate = "everarcade-rustrig-combat"
certification = "PASS"
authority = "world"
receipt = "CombatReceipt"
```

The mutation reads combat-relevant world state and writes the combat state plus canonical receipts.

## Authority Rules

The reference contract assigns `combat.attack()` to `world` authority. The World Contract is responsible for declaring allowed attack types and the RustRig validates that each attack uses one of those declared types. Runtime integrations can layer caller-control checks before invoking the certified RustRig, but they must not alter damage or receipt semantics.

## State Transition

Inputs are:

- `attacker_id`
- `target_id`
- `damage`
- `attack_type`
- `tick`

Validation requires an existing live attacker, an existing live target, positive damage, damage no greater than the configured maximum, and an attack type declared by the World Contract.

The initial deterministic damage model is intentionally simple:

```text
final_damage = damage
new_health = max(current_health - damage, 0)
```

No RNG, critical hits, floating point arithmetic, timestamps, or hidden state are allowed. When target health reaches `0`, `alive` is set to `false` deterministically.

## Receipt Generation

`CombatReceipt` records the exact transition:

- deterministic `receipt_id`
- attacker and target IDs
- declared damage and attack type
- target `health_before` and `health_after`
- `tick`
- input hash
- pre-state root
- post-state root
- authority and result

The receipt must match the state transition exactly. Applied damage is always the declared damage, with health flooring at zero for overkill.

## Replay Behavior

Replay starts from an initial `CombatState` and re-applies the same ordered `CombatInput` sequence. Because validation, damage, death, receipt generation, and root computation are deterministic, replay produces identical combat logs, receipts, entity health, alive flags, and state roots.

## Certification Requirements

The certified combat primitive satisfies:

- **RR-COMBAT-001 Combat Determinism**: same input and state produce the same receipt and state.
- **RR-COMBAT-002 Damage Integrity**: applied damage equals declared damage.
- **RR-COMBAT-003 Health Floor**: health never becomes negative.
- **RR-COMBAT-004 Death Consistency**: death is derived only from resulting health.
- **RR-COMBAT-005 Receipt Integrity**: receipt fields exactly match the transition.
- **RR-COMBAT-006 Replay Equivalence**: replay produces identical state.
- **RR-COMBAT-007 Root Equivalence**: state roots remain identical for identical transitions.
- **RR-COMBAT-008 JS ↔ Runtime Equivalence**: external implementations must match receipts and roots.

Certification output:

```text
RUSTRIG COMBAT CERTIFICATION: PASS
```

## Planned Deterministic Extensions

Future combat RustRigs must inherit this certification model:

- `combat.heal()`
- `combat.revive()`
- `combat.buff()`
- `combat.debuff()`
- `combat.aoe_attack()`
- `combat.projectile()`
- `combat.status_effect()`
