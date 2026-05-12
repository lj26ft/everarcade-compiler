use crate::economics::ResourceUsage;
pub fn simulate_usage(samples: &[ResourceUsage]) -> ResourceUsage { samples.iter().copied().fold(ResourceUsage::default(), |a, b| a.saturating_add(b)) }
