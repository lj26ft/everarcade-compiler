use std::collections::BTreeSet;

use contract_api::protocol_records::RecordFields;
use contract_api::rustrig::RustrigContext;

use super::error::{Result, RustrigRuntimeError};
use super::executor::stable_hash;
use super::kernel::{ExecutionRequest, RustrigKernel};
use super::receipt::CompositionReceipt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineStep {
    pub id: String,
    pub payload: RecordFields,
}

pub fn execute_pipeline(
    kernel: &mut RustrigKernel,
    pipeline_id: &str,
    version: &str,
    context: RustrigContext,
    steps: Vec<PipelineStep>,
) -> Result<CompositionReceipt> {
    let mut seen = BTreeSet::new();
    let mut receipts = Vec::new();
    let mut records = Vec::new();
    for step in steps {
        if !seen.insert(step.id.clone()) {
            return Err(RustrigRuntimeError::InvalidPipeline(format!(
                "cycle or duplicate step {}",
                step.id
            )));
        }
        let execution = kernel.execute(ExecutionRequest {
            rustrig_id: step.id,
            version: version.to_string(),
            context: context.clone(),
            payload: step.payload,
        })?;
        records.extend(execution.output.records.clone());
        receipts.push(execution.receipt);
    }
    Ok(CompositionReceipt {
        pipeline_id: pipeline_id.to_string(),
        composition_root: stable_hash(&(pipeline_id, &receipts, &records)),
        receipts,
        records,
    })
}
