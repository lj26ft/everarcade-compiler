use execution_core::render_bridge::{ProjectedFrameState, ProjectedInventoryFrame};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldViewConfig {
    pub width: i64,
    pub height: i64,
}

impl Default for WorldViewConfig {
    fn default() -> Self {
        Self {
            width: 16,
            height: 8,
        }
    }
}

pub fn render_world_ascii(state: &ProjectedFrameState, cfg: &WorldViewConfig) -> String {
    let mut grid = vec![vec!['.'; cfg.width as usize]; cfg.height as usize];
    for entity in &state.entities {
        let x = entity.x.rem_euclid(cfg.width) as usize;
        let y = entity.y.rem_euclid(cfg.height) as usize;
        grid[y][x] = 'E';
    }
    let mut out = String::new();
    out.push_str(&format!(
        "tick={} world_root={}\n",
        state.world.tick, state.world.world_root
    ));
    for row in grid {
        out.push_str(&row.into_iter().collect::<String>());
        out.push('\n');
    }
    if !state.inventory.is_empty() {
        out.push_str("inventory: ");
        out.push_str(&render_inventory_markers(&state.inventory));
        out.push('\n');
    }
    out
}

pub fn render_inventory_markers(inventory: &[ProjectedInventoryFrame]) -> String {
    inventory
        .iter()
        .map(|i| format!("{}:{}", i.owner, i.inventory_root))
        .collect::<Vec<_>>()
        .join(",")
}
