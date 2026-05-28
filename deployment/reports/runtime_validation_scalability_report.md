# Runtime Validation Scalability Report
- partitioned_validation: enabled
- cargo_build_jobs_default: 1
- offline_locked_supported: yes
- disk_preflight: scripts/run_validation_disk_preflight.sh
- cleanup_before_validation: --cleanup-before-validation
- log: runtime/logs/partitioned_workspace_validation.log
- monolithic_workspace_test: intentionally avoided for this milestone
