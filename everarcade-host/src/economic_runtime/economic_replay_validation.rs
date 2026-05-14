pub fn validate_economic_replay(expected: [u8; 32], observed: [u8; 32]) -> Result<(), &'static str> { if expected == observed { Ok(()) } else { Err("economic replay divergence") } }
