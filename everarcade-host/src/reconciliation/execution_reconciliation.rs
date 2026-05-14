pub type Hash = [u8; 32];

pub fn select_highest_valid_replay_continuity(candidates: &[Hash]) -> Option<Hash> {
    candidates.iter().copied().max()
}
