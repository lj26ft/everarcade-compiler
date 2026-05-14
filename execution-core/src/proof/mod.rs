pub mod aggregated_proof;
pub mod aggregation;
pub mod commitment;
pub mod compression;
pub mod execution_trace;
pub mod proof;
pub mod proof_aggregation;
pub mod proof_chain;
pub mod proof_checkpoint;
pub mod proof_commitment;
pub mod proof_receipt;
pub mod proof_transport;
pub mod proof_validation;
pub mod prover;
pub mod recursive;
pub mod recursive_proof_placeholder;
pub mod transcript;
pub mod verifier;

pub use aggregation::{aggregate_proofs, AggregateProof};
pub use commitment::{
    execution_commitment, receipt_commitment, snapshot_commitment, trace_commitment,
};
pub use execution_trace::{ExecutionTrace, TraceNode, TraceTransition};
pub use proof::ExecutionProof;
pub use proof_checkpoint::{checkpoint_root, ProofCheckpoint};
pub use proof_receipt::ProofReceipt;
pub use proof_transport::{chunk_proof, reconstruct_proof, ProofChunk};
pub use prover::deterministic_prove;
pub use recursive::{compose_lineage, RecursiveProofLineage};
pub use transcript::{transcript_hash, Transcript};
pub use verifier::{
    verify_epoch, verify_execution_root, verify_proof_integrity, verify_receipt_binding,
};
