# RustRig Standard Library v1

RustRigs are deterministic world mutation kernels. Each mutation defines state model, input model, validation rules, transition rules, receipt model, invariants, property targets, replay behavior, and root behavior.

Maturity levels: CERTIFIED, CANDIDATE, EXPERIMENTAL, PLANNED. The full library is not claimed formally proven.

## Certified mutations

- inventory.transfer
- combat.attack
- market.trade
- governance.vote

## Candidate domains

- **identity**: identity.join, identity.leave
- **movement**: position.move
- **world**: world.spawn, world.despawn
- **combat**: combat.attack, combat.heal, combat.revive, combat.apply_status, combat.remove_status, combat.area_attack
- **inventory**: inventory.transfer, inventory.pickup, inventory.drop, inventory.equip, inventory.unequip, inventory.consume
- **resources**: resource.harvest, resource.regenerate
- **crafting**: crafting.craft
- **structures**: structure.build, structure.decay, structure.repair
- **market**: market.trade, market.list, market.cancel, market.bid, market.accept_bid, market.settle_auction, market.collect_fee
- **governance**: governance.vote, governance.create_proposal, governance.finalize_proposal, governance.cancel_proposal, governance.delegate_vote, governance.amend_policy
- **factions**: faction.create, faction.join, faction.leave
- **quests**: quest.start, quest.progress, quest.complete
- **continuity**: continuity.advance, continuity.record_event, continuity.create_ruin
- **operations**: operation.checkpoint, operation.restore, operation.migrate
