use super::{
    checkpoint::CheckpointRecord,
    error::CommitError,
    journal::JournalEntry,
    receipt::ExecutionReceipt,
    state_diff::{
        canonicalize_changes, compute_state_root, state_diff_hash, StateChange, StateDiff,
    },
};

#[derive(Debug, Clone)]
pub struct CommitInput {
    pub contract_id: String,
    pub execution_id: String,
    pub previous_state_root: [u8; 32],
    pub state_changes: Vec<StateChange>,
    pub fuel_used: u64,
    pub previous_entry_hash: [u8; 32],
    pub expected_sequence_number: u64,
    pub is_noop: bool,
}
#[derive(Debug, Clone)]
pub struct CommitOutput {
    pub state_diff: StateDiff,
    pub receipt: ExecutionReceipt,
    pub journal_entry: JournalEntry,
    pub checkpoint: CheckpointRecord,
}

pub fn commit_execution(input: CommitInput) -> Result<CommitOutput, CommitError> {
    let sorted = canonicalize_changes(input.state_changes, input.is_noop)?;
    let new_state_root = compute_state_root(input.previous_state_root, &sorted);
    let state_diff = StateDiff {
        changes: sorted,
        previous_state_root: input.previous_state_root,
        new_state_root,
    };
    let diff_hash = state_diff_hash(&state_diff);
    let mut receipt = ExecutionReceipt {
        contract_id: input.contract_id,
        execution_id: input.execution_id,
        previous_state_root: state_diff.previous_state_root,
        new_state_root: state_diff.new_state_root,
        state_diff_hash: diff_hash,
        fuel_used: input.fuel_used,
        receipt_hash: [0; 32],
        continuity_hash: input.previous_entry_hash,
    };
    receipt.receipt_hash = receipt.immutable_hash();
    let mut journal = JournalEntry {
        sequence_number: input.expected_sequence_number,
        previous_entry_hash: input.previous_entry_hash,
        receipt_hash: receipt.receipt_hash,
        state_diff_hash: diff_hash,
        checkpoint_hash: [0; 32],
        entry_hash: [0; 32],
    };
    let checkpoint_hint =
        CheckpointRecord::from_parts(state_diff.new_state_root, journal.sequence_number, [0; 32]);
    journal.checkpoint_hash = checkpoint_hint.checkpoint_hash;
    journal.entry_hash = journal.compute_hash();
    let checkpoint = CheckpointRecord::from_parts(
        state_diff.new_state_root,
        journal.sequence_number,
        journal.entry_hash,
    );
    Ok(CommitOutput {
        state_diff,
        receipt,
        journal_entry: journal,
        checkpoint,
    })
}
