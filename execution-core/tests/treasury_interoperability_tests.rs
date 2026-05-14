use execution_core::treaty::{
    fiscal_interoperability::coordinate_fiscal, treasury_exchange::exchange_treasury,
};
#[test]
fn treasury_interoperability_stable() {
    let t = exchange_treasury([5; 32], [9; 32]);
    let f = coordinate_fiscal([1; 32], [2; 32]);
    assert_ne!(t, [0; 32]);
    assert_ne!(f, [0; 32]);
}
