pub fn total_supply(prior:u128, delta:i128)->u128 { if delta.is_negative() { prior.saturating_sub(delta.unsigned_abs()) } else { prior.saturating_add(delta as u128) } }
