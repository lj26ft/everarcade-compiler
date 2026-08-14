# Continuum Runner: Arena

The retained P5 source uses the production `world-evr` compiler and full PTW profile. Repeated compilation must produce byte-identical Runtime IR and `world.evr` archives.

The current full-primitives profile exposes `player.join`, `entity.move`, `interaction.activate`, `item.pickup`, `item.use`, `combat.attack`, and `world.respawn`. It does not expose distinct jump, look, projectile, reload, ability, damage-resolution, death-resolution, team, or score actions. P5 records this honestly as the remaining compiler/Authority combat-contract gap rather than routing those actions through benchmark-local state.
