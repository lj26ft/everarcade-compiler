#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FederationEvent {
    ProposalCreated { proposal_id: String, proposer: String, body_hash: String },
    VoteSubmitted { proposal_id: String, voter: String, approve: bool },
    ResolutionFinalized { proposal_id: String, resolution_id: String, approved: bool },
    TreatyEstablished { treaty_id: String, terms_hash: String },
    TreatySuperseded { previous_treaty_id: String, new_treaty_id: String, terms_hash: String },
    ConstitutionAmended { amendment_id: String, constitutional_root: String },
    MemberJoined { member_id: String },
    MemberExited { member_id: String },
    FederationForked { fork_id: String, reason_hash: String },
    FederationMigrated { migration_id: String, continuity_root: String },
}
