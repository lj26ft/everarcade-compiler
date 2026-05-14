pub fn sovereignty_index(
    vendor_count: u64,
    independent_nodes: u64,
    migratable_packages: u64,
) -> u64 {
    vendor_count
        .saturating_mul(5)
        .saturating_add(independent_nodes.saturating_mul(10))
        .saturating_add(migratable_packages.saturating_mul(3))
}
