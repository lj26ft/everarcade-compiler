use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostEstimate {
    pub games: u32,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub storage_gb: u64,
    pub bandwidth_mbps: u64,
    pub lease_count: u32,
    pub estimated_cost_usd: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArenaVanguardCostModel {
    pub single_game: CostEstimate,
    pub ten_games: CostEstimate,
    pub hundred_games: CostEstimate,
    pub thousand_games: CostEstimate,
}

pub fn estimate_games(games: u32) -> CostEstimate {
    CostEstimate {
        games,
        cpu_cores: games * 4,
        memory_mb: u64::from(games) * 8192,
        storage_gb: u64::from(games) * 200,
        bandwidth_mbps: u64::from(games) * 100,
        lease_count: games,
        estimated_cost_usd: f64::from(games) * 42.50,
    }
}
pub fn arena_vanguard_cost_model() -> ArenaVanguardCostModel {
    ArenaVanguardCostModel {
        single_game: estimate_games(1),
        ten_games: estimate_games(10),
        hundred_games: estimate_games(100),
        thousand_games: estimate_games(1000),
    }
}
