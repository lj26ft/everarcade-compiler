#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DivergenceView { pub divergent: bool, pub first_tick: Option<u64> }

pub fn visualize_divergence(expected: &[&str], actual: &[&str]) -> DivergenceView {
    let first_tick = expected.iter().zip(actual.iter()).position(|(a,b)| a != b).map(|i| i as u64);
    DivergenceView { divergent: first_tick.is_some() || expected.len() != actual.len(), first_tick }
}
