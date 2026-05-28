#![allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CivilizationHealthMetric {
    pub faction_continuity: &'static str,
    pub diplomacy_continuity: &'static str,
    pub ecological_continuity: &'static str,
    pub societal_continuity: &'static str,
    pub social_memory_continuity: &'static str,
    pub replay_continuity: &'static str,
    pub federation_continuity: &'static str,
}

pub fn health() -> CivilizationHealthMetric {
    CivilizationHealthMetric {
        faction_continuity: "preserved",
        diplomacy_continuity: "preserved",
        ecological_continuity: "preserved",
        societal_continuity: "preserved",
        social_memory_continuity: "append-only",
        replay_continuity: "append-only",
        federation_continuity: "preserved",
    }
}
