pub fn detect_corruption(input: &str) -> bool {
    matches!(
        input,
        "invalid-order"
            | "replay-corruption"
            | "checkpoint-forgery"
            | "partition-corruption"
            | "artifact-corruption"
            | "invalid-ancestry"
            | "replay-divergence"
            | "timeout-corruption"
            | "batch-corruption"
            | "signature-mismatch"
    )
}
