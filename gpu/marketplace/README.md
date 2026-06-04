# GPU Marketplace v0.1

The GPU Marketplace is a deterministic, replay-safe economic network scaffold
for non-authoritative projection compute providers. It consumes GPU Runtime jobs
and renderer projection roots, assigns work to providers, verifies submitted
artifacts, and derives settlement-intent evidence without making payments or
changing authoritative lease state.

The executable model is `marketplace_model.sh`; validation and certification are
provided by `scripts/validate_gpu_marketplace.sh` and
`scripts/certify_gpu_marketplace.sh`.
