use everarcade_host::governance_security::malicious_proposal::reject_if_invalid;
#[test] fn malicious_rejected() { assert!(reject_if_invalid(false).is_err()); }
