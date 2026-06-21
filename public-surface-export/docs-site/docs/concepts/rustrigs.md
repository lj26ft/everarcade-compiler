# What Are RustRigs?

RustRigs are canonical gameplay mutation libraries for EverArcade worlds.

They help developers implement common world actions without rebuilding every rule from scratch.

## Examples

```rust
combat.attack(attacker, target)
inventory.transfer(from, to, item)
market.trade(order, buyer)
quest.complete(player, quest)
```

## Why developers use them

RustRigs improve productivity by turning recurring gameplay patterns into reusable, reviewable building blocks.

## What they are not

RustRigs do not replace a world's design. They provide tested patterns that a World Contract can compose into its own rulebook.
