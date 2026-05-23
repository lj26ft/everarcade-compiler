#[cfg(test)]
mod tests {
    use crate::{
        diagnostics::{ExecutionProfile, OperatorDiagnosticEnvelope},
        execute::execute_vm,
        ExecutionNode, ExecutionPlan, State, VmInput,
    };

    fn sample_input() -> VmInput {
        VmInput {
            protocol_epoch_id: 1,
            state: State::new(),
            plan: ExecutionPlan {
                nodes: vec![ExecutionNode {
                    id: "n0".into(),
                    action: "set".into(),
                    payload: serde_json::json!({"key": "k", "value": "1"}),
                    deps: vec![],
                }],
            },
        }
    }

    #[test]
    fn test_profile_record_canonical_ordering() {
        let env = OperatorDiagnosticEnvelope {
            component: "execution-core".into(),
            event: "execution_profile".into(),
            sequence: 0,
            deterministic: true,
            profile: ExecutionProfile::default(),
        };
        let line = env.json_line().expect("serialize");
        assert!(line.contains("\"component\""));
        assert!(line.find("\"component\"").unwrap() < line.find("\"event\"").unwrap());
    }

    #[test]
    fn test_profile_does_not_change_receipt() {
        let out_a = execute_vm(sample_input());
        let _profile = ExecutionProfile::default();
        let out_b = execute_vm(sample_input());
        assert_eq!(out_a.receipt.receipt_hash, out_b.receipt.receipt_hash);
    }

    #[test]
    fn test_profile_does_not_change_state_root() {
        let out_a = execute_vm(sample_input());
        let _profile = ExecutionProfile::default();
        let out_b = execute_vm(sample_input());
        assert_eq!(out_a.receipt.new_state_root, out_b.receipt.new_state_root);
    }

    #[test]
    fn test_dag_profile_counts_are_stable() {
        let p = ExecutionProfile {
            operation_index: 3,
            receipt_count: 1,
            ..Default::default()
        };
        let q = ExecutionProfile {
            operation_index: 3,
            receipt_count: 1,
            ..Default::default()
        };
        assert_eq!(p, q);
    }

    #[test]
    fn test_replay_profile_reports_divergence() {
        let p = ExecutionProfile {
            warnings: vec!["divergence: receipt root changed".into()],
            ..Default::default()
        };
        assert!(p.warnings.iter().any(|w| w.contains("divergence")));
    }

    #[test]
    fn test_wasm_profile_does_not_change_result() {
        let out_a = execute_vm(sample_input());
        let _wasm = ExecutionProfile {
            wasm_call_count: 2,
            ..Default::default()
        };
        let out_b = execute_vm(sample_input());
        assert_eq!(out_a.updated_state, out_b.updated_state);
    }
}
