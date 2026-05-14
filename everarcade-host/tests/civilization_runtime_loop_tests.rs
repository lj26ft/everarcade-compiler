use everarcade_host::civilization_runtime::runtime_loop::execute_runtime_loop;
#[test]
fn civilization_loop_reconstructible() {
    let c = execute_runtime_loop([7; 32], [7; 32], [1; 32], [2; 32]);
    assert_eq!(c.civilization_root, [0; 32]);
}
