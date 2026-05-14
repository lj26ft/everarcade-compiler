use crate::fiscal::fiscal_policy::Hash;
pub fn simulate_fiscal_step(root: Hash, pressure: u8) -> Hash {
    let mut o = root;
    o[0] = o[0].wrapping_add(pressure);
    o
}
