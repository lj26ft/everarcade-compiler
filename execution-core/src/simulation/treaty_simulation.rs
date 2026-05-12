pub fn treaty_continuity_valid(history: &[[u8; 32]]) -> bool { !history.is_empty() && history.windows(2).all(|w| w[0] != [0; 32] && w[1] != [0; 32]) }
