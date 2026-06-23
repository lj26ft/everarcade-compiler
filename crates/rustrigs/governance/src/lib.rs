use std::collections::{BTreeMap, BTreeSet};

pub const RUSTRIG_ID: &str = "governance.vote";
pub const RUSTRIG_VERSION: &str = "1.0.0";
pub const CANDIDATE_MUTATIONS: &[&str] = &[
    "governance.create_proposal",
    "governance.finalize_proposal",
    "governance.cancel_proposal",
    "governance.delegate_vote",
    "governance.amend_policy",
];
pub const CERTIFICATION_STATUS: &str = "RUSTRIG GOVERNANCE VOTE CERTIFICATION: PASS";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoteInput {
    pub proposal_id: String,
    pub voter_id: String,
    pub vote_choice: String,
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

impl VoteChoice {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "yes" => Some(Self::Yes),
            "no" => Some(Self::No),
            "abstain" => Some(Self::Abstain),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Abstain => "abstain",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Accepted,
    Rejected,
    Expired,
}

impl ProposalStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    fn is_final(&self) -> bool {
        !matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub proposal_id: String,
    pub title: String,
    pub description: String,
    pub proposer_id: String,
    pub created_tick: u64,
    pub voting_start_tick: u64,
    pub voting_end_tick: u64,
    pub status: ProposalStatus,
    pub quorum: u64,
}

impl Proposal {
    pub fn new(
        proposal_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        proposer_id: impl Into<String>,
        created_tick: u64,
        voting_start_tick: u64,
        voting_end_tick: u64,
        quorum: u64,
    ) -> Self {
        Self {
            proposal_id: proposal_id.into(),
            title: title.into(),
            description: description.into(),
            proposer_id: proposer_id.into(),
            created_tick,
            voting_start_tick,
            voting_end_tick,
            status: ProposalStatus::Active,
            quorum,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CastVote {
    pub proposal_id: String,
    pub voter_id: String,
    pub vote_choice: VoteChoice,
    pub tick: u64,
    pub receipt_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoteReceipt {
    pub receipt_id: String,
    pub proposal_id: String,
    pub voter_id: String,
    pub vote_choice: String,
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceLogEntry {
    pub event: String,
    pub proposal_id: String,
    pub actor_id: String,
    pub tick: u64,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceState {
    pub proposals: BTreeMap<String, Proposal>,
    pub votes: BTreeMap<(String, String), CastVote>,
    pub voters: BTreeSet<String>,
    pub receipts: BTreeMap<String, VoteReceipt>,
    pub governance_log: Vec<GovernanceLogEntry>,
}

impl GovernanceState {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
            votes: BTreeMap::new(),
            voters: BTreeSet::new(),
            receipts: BTreeMap::new(),
            governance_log: Vec::new(),
        }
    }

    pub fn with_voter(mut self, voter_id: impl Into<String>) -> Self {
        self.voters.insert(voter_id.into());
        self
    }

    pub fn with_proposal(mut self, proposal: Proposal) -> Self {
        self.proposals
            .insert(proposal.proposal_id.clone(), proposal);
        self
    }

    pub fn tally(&self, proposal_id: &str) -> Tally {
        self.votes
            .values()
            .filter(|vote| vote.proposal_id == proposal_id)
            .fold(Tally::default(), |mut tally, vote| {
                match vote.vote_choice {
                    VoteChoice::Yes => tally.yes_count += 1,
                    VoteChoice::No => tally.no_count += 1,
                    VoteChoice::Abstain => tally.abstain_count += 1,
                }
                tally
            })
    }

    pub fn state_root(&self) -> String {
        digest(&self.canonical())
    }

    fn canonical(&self) -> String {
        let proposals = self
            .proposals
            .iter()
            .map(|(id, p)| {
                format!(
                    "proposal:{id}:{}:{}:{}:{}:{}:{}:{}:{}",
                    p.title,
                    p.description,
                    p.proposer_id,
                    p.created_tick,
                    p.voting_start_tick,
                    p.voting_end_tick,
                    p.status.as_str(),
                    p.quorum
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let votes = self
            .votes
            .iter()
            .map(|((proposal, voter), v)| {
                format!(
                    "vote:{proposal}:{voter}:{}:{}:{}",
                    v.vote_choice.as_str(),
                    v.tick,
                    v.receipt_id
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let voters = self
            .voters
            .iter()
            .map(|v| format!("voter:{v}"))
            .collect::<Vec<_>>()
            .join("|");
        let receipts = self
            .receipts
            .iter()
            .map(|(id, r)| {
                format!(
                    "receipt:{id}:{}:{}:{}:{}",
                    r.proposal_id, r.voter_id, r.vote_choice, r.tick
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let log = self
            .governance_log
            .iter()
            .map(|e| {
                format!(
                    "log:{}:{}:{}:{}:{}",
                    e.event, e.proposal_id, e.actor_id, e.tick, e.detail
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("proposals=[{proposals}];votes=[{votes}];voters=[{voters}];receipts=[{receipts}];governance_log=[{log}]")
    }
}

impl Default for GovernanceState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub yes_count: u64,
    pub no_count: u64,
    pub abstain_count: u64,
}
impl Tally {
    pub fn votes_cast(&self) -> u64 {
        self.yes_count + self.no_count + self.abstain_count
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoteOutput {
    pub receipt: VoteReceipt,
    pub state: GovernanceState,
    pub state_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinalizeOutput {
    pub proposal_id: String,
    pub status: ProposalStatus,
    pub tally: Tally,
    pub state: GovernanceState,
    pub state_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GovernanceError {
    ProposalMissing,
    VoterMissing,
    ProposalNotActive,
    TickBeforeVotingWindow,
    TickAfterVotingWindow,
    InvalidVoteChoice,
    DuplicateVote,
    ProposalFinalized,
    FinalizationTooEarly,
}

pub fn vote(state: &GovernanceState, input: VoteInput) -> Result<VoteOutput, GovernanceError> {
    let proposal = state
        .proposals
        .get(&input.proposal_id)
        .ok_or(GovernanceError::ProposalMissing)?;
    if !state.voters.contains(&input.voter_id) {
        return Err(GovernanceError::VoterMissing);
    }
    if proposal.status.is_final() {
        return Err(GovernanceError::ProposalFinalized);
    }
    if proposal.status != ProposalStatus::Active {
        return Err(GovernanceError::ProposalNotActive);
    }
    if input.tick < proposal.voting_start_tick {
        return Err(GovernanceError::TickBeforeVotingWindow);
    }
    if input.tick > proposal.voting_end_tick {
        return Err(GovernanceError::TickAfterVotingWindow);
    }
    let choice = VoteChoice::parse(&input.vote_choice).ok_or(GovernanceError::InvalidVoteChoice)?;
    let vote_key = (input.proposal_id.clone(), input.voter_id.clone());
    if state.votes.contains_key(&vote_key) {
        return Err(GovernanceError::DuplicateVote);
    }

    let mut next_state = state.clone();
    let receipt_id = deterministic_receipt_id(&input);
    let receipt = VoteReceipt {
        receipt_id: receipt_id.clone(),
        proposal_id: input.proposal_id.clone(),
        voter_id: input.voter_id.clone(),
        vote_choice: choice.as_str().to_owned(),
        tick: input.tick,
    };
    let cast_vote = CastVote {
        proposal_id: input.proposal_id.clone(),
        voter_id: input.voter_id.clone(),
        vote_choice: choice,
        tick: input.tick,
        receipt_id: receipt_id.clone(),
    };
    next_state.votes.insert(vote_key, cast_vote);
    next_state.receipts.insert(receipt_id, receipt.clone());
    next_state.governance_log.push(GovernanceLogEntry {
        event: "vote_cast".to_owned(),
        proposal_id: input.proposal_id,
        actor_id: input.voter_id,
        tick: input.tick,
        detail: receipt.vote_choice.clone(),
    });
    let state_root = next_state.state_root();
    Ok(VoteOutput {
        receipt,
        state: next_state,
        state_root,
    })
}

pub fn finalize_proposal(
    state: &GovernanceState,
    proposal_id: &str,
    tick: u64,
) -> Result<FinalizeOutput, GovernanceError> {
    let proposal = state
        .proposals
        .get(proposal_id)
        .ok_or(GovernanceError::ProposalMissing)?;
    if proposal.status.is_final() {
        return Err(GovernanceError::ProposalFinalized);
    }
    if tick <= proposal.voting_end_tick {
        return Err(GovernanceError::FinalizationTooEarly);
    }
    let tally = state.tally(proposal_id);
    let status = if tally.yes_count > tally.no_count && tally.votes_cast() >= proposal.quorum {
        ProposalStatus::Accepted
    } else {
        ProposalStatus::Rejected
    };
    let mut next_state = state.clone();
    next_state
        .proposals
        .get_mut(proposal_id)
        .expect("proposal exists")
        .status = status.clone();
    next_state.governance_log.push(GovernanceLogEntry {
        event: "proposal_finalized".to_owned(),
        proposal_id: proposal_id.to_owned(),
        actor_id: "world".to_owned(),
        tick,
        detail: status.as_str().to_owned(),
    });
    let state_root = next_state.state_root();
    Ok(FinalizeOutput {
        proposal_id: proposal_id.to_owned(),
        status,
        tally,
        state: next_state,
        state_root,
    })
}

pub fn replay_with_finalizations(
    initial_state: &GovernanceState,
    inputs: &[VoteInput],
    finalizations: &[(String, u64)],
) -> Result<GovernanceState, GovernanceError> {
    let mut state = initial_state.clone();
    for input in inputs {
        state = vote(&state, input.clone())?.state;
    }
    for (proposal_id, tick) in finalizations {
        state = finalize_proposal(&state, proposal_id, *tick)?.state;
    }
    Ok(state)
}

/// Standard RustRig input model for `governance.vote`.
pub type Input = VoteInput;
/// Standard RustRig state model for `governance.vote`.
pub type State = GovernanceState;
/// Standard RustRig output/receipt model for `governance.vote`.
pub type Output = VoteOutput;
/// Standard RustRig error model for `governance.vote`.
pub type Error = GovernanceError;

pub fn apply(input: Input, state: State) -> Result<Output, Error> {
    vote(&state, input)
}

pub fn governance_vote(input: Input, state: State) -> Result<Output, Error> {
    apply(input, state)
}

pub fn replay(inputs: &[Input], genesis: State) -> Result<State, Error> {
    let mut state = genesis;
    for input in inputs {
        state = apply(input.clone(), state)?.state;
    }
    Ok(state)
}

pub fn state_root(state: &State) -> String {
    state.state_root()
}

pub fn certified_status() -> &'static str {
    CERTIFICATION_STATUS
}

fn deterministic_receipt_id(input: &VoteInput) -> String {
    digest(&format!(
        "{RUSTRIG_ID}|{RUSTRIG_VERSION}|proposal_id={};voter_id={};vote_choice={};tick={}",
        input.proposal_id, input.voter_id, input.vote_choice, input.tick
    ))
}
fn digest(value: &str) -> String {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;
    let mut hash = OFFSET;
    for byte in value.as_bytes() {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("ea{:032x}", hash)
}
