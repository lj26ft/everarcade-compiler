pub type Hash=[u8;32];
pub fn issuance_root(epoch:u64, amount:u64)->Hash { let mut o=[0;32]; o[..8].copy_from_slice(&epoch.to_le_bytes()); o[8..16].copy_from_slice(&amount.to_le_bytes()); o }
