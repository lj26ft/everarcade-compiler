use everarcade_host::distributed_execution::partition_scheduler::schedule_partitions;

#[test]
fn partition_determinism() {
    let package = [1u8; 32];
    let window = [2u8; 32];
    let inputs = vec![[3u8; 32], [4u8; 32]];
    let operators = vec![[9u8; 32], [8u8; 32]];
    assert_eq!(
        schedule_partitions(package, window, &inputs, &operators),
        schedule_partitions(package, window, &inputs, &operators)
    );
}
