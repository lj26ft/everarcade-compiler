pub type Hash = [u8; 32];
pub fn reputation_penalty(current: i32, violation_count: u32) -> i32 { current - violation_count as i32 }
