use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub type Amount = i128;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTreasuryRecord {
    pub world_id: String,
    pub treasury_id: String,
    pub treasury_root: String,
    pub balance: BTreeMap<String, Amount>,
    pub income_streams: Vec<RevenueEvent>,
    pub expense_streams: Vec<ExpenseEvent>,
    pub governance_model: TreasuryGovernanceModel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreasuryGovernanceModel {
    OperatorControlled,
    CouncilControlled,
    CommunityControlled,
    Hybrid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueType {
    MarketplaceFees,
    CapabilityRoyalties,
    SubscriptionRevenue,
    TransactionFees,
    GovernanceFees,
    LicensingRevenue,
    EventRevenue,
    DonationRevenue,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevenueEvent {
    pub revenue_id: String,
    pub world_id: String,
    pub revenue_type: RevenueType,
    pub asset: String,
    pub amount: Amount,
    pub payer_id: String,
    pub attribution: String,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpenseType {
    ContributorPayments,
    OperatorCompensation,
    InfrastructureCosts,
    ModerationCosts,
    GovernanceOperations,
    WorldEvents,
    TreasuryGrants,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpenseEvent {
    pub expense_id: String,
    pub world_id: String,
    pub expense_type: ExpenseType,
    pub asset: String,
    pub amount: Amount,
    pub recipient_id: String,
    pub artifact_id: String,
    pub governance_action_id: String,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentType {
    FixedPayment,
    RevenueShare,
    Royalty,
    TreasuryGrant,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompensationArtifact {
    pub payment_id: String,
    pub contributor_id: String,
    pub artifact_id: String,
    pub payment_type: PaymentType,
    pub asset: String,
    pub amount: Amount,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRoyaltyEvent {
    pub royalty_id: String,
    pub capability_id: String,
    pub creator_id: String,
    pub world_id: String,
    pub usage_units: u64,
    pub asset: String,
    pub amount: Amount,
    pub attribution_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreasuryGovernanceActionType {
    BudgetApproval,
    GrantApproval,
    CompensationApproval,
    TreasuryAllocation,
    EmergencyExpenditure,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreasuryGovernanceAction {
    pub action_id: String,
    pub action_type: TreasuryGovernanceActionType,
    pub proposal_id: String,
    pub approved_by: Vec<String>,
    pub execution_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetCategory {
    Development,
    Infrastructure,
    Moderation,
    Events,
    TreasuryReserve,
    Education,
    Growth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub category: BudgetCategory,
    pub asset: String,
    pub amount: Amount,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetArtifact {
    pub budget_id: String,
    pub fiscal_period: String,
    pub allocations: Vec<BudgetAllocation>,
    pub version: u64,
    pub governance_action_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantType {
    ContributorGrant,
    CapabilityGrant,
    InfrastructureGrant,
    ResearchGrant,
    CommunityGrant,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantStage {
    Proposal,
    Review,
    Vote,
    Approval,
    Disbursement,
    Reporting,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantArtifact {
    pub grant_id: String,
    pub grant_type: GrantType,
    pub stage: GrantStage,
    pub recipient_id: String,
    pub asset: String,
    pub amount: Amount,
    pub outcome_report: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TreasuryHealthMetrics {
    pub revenue: Amount,
    pub expenses: Amount,
    pub reserve_ratio_bps: i128,
    pub contributor_payouts: Amount,
    pub grant_performance_bps: i128,
    pub revenue_growth_bps: i128,
    pub treasury_runway_periods: i128,
    pub dependency_concentration_bps: i128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorDuty {
    RuntimeHosting,
    WorldAvailability,
    UpgradeManagement,
    SecurityOperations,
    GovernanceExecution,
    TreasuryExecution,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorActionRecord {
    pub action_id: String,
    pub operator_id: String,
    pub duty: OperatorDuty,
    pub artifact_id: String,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorCompensationSource {
    TreasuryAllocation,
    HostingFees,
    GovernanceAllocation,
    MarketplaceRevenueShare,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreasuryReplayData {
    pub initial: WorldTreasuryRecord,
    pub revenues: Vec<RevenueEvent>,
    pub expenses: Vec<ExpenseEvent>,
    pub grants: Vec<GrantArtifact>,
    pub compensation: Vec<CompensationArtifact>,
    pub governance: Vec<TreasuryGovernanceAction>,
}

impl WorldTreasuryRecord {
    pub fn new(world_id: impl Into<String>, governance_model: TreasuryGovernanceModel) -> Self {
        let world_id = world_id.into();
        let treasury_id = stable_hash(&["treasury", &world_id]);
        let mut record = Self {
            world_id,
            treasury_id,
            treasury_root: String::new(),
            balance: BTreeMap::new(),
            income_streams: vec![],
            expense_streams: vec![],
            governance_model,
        };
        record.refresh_root();
        record
    }

    pub fn apply_revenue(&mut self, event: RevenueEvent) {
        *self.balance.entry(event.asset.clone()).or_default() += event.amount;
        self.income_streams.push(event);
        self.refresh_root();
    }
    pub fn apply_expense(&mut self, event: ExpenseEvent) -> Result<(), String> {
        let bal = self.balance.entry(event.asset.clone()).or_default();
        if *bal < event.amount {
            return Err("treasury balance would become negative".to_owned());
        }
        *bal -= event.amount;
        self.expense_streams.push(event);
        self.refresh_root();
        Ok(())
    }
    pub fn refresh_root(&mut self) {
        self.treasury_root = treasury_root(self);
    }
}

pub fn fixed_payment(
    id: &str,
    contributor_id: &str,
    artifact_id: &str,
    asset: &str,
    amount: Amount,
) -> CompensationArtifact {
    CompensationArtifact {
        payment_id: id.to_owned(),
        contributor_id: contributor_id.to_owned(),
        artifact_id: artifact_id.to_owned(),
        payment_type: PaymentType::FixedPayment,
        asset: asset.to_owned(),
        amount,
        source: "treasury".to_owned(),
    }
}
pub fn revenue_share(total_revenue: Amount, share_bps: i128) -> Amount {
    total_revenue * share_bps / 10_000
}
pub fn royalty_payment(usage_units: u64, unit_price: Amount, royalty_bps: i128) -> Amount {
    usage_units as Amount * unit_price * royalty_bps / 10_000
}

pub fn replay_treasury(data: &TreasuryReplayData) -> Result<WorldTreasuryRecord, String> {
    let mut state = data.initial.clone();
    state.income_streams.clear();
    state.expense_streams.clear();
    state.refresh_root();
    for r in data.revenues.clone() {
        state.apply_revenue(r);
    }
    for e in data.expenses.clone() {
        state.apply_expense(e)?;
    }
    Ok(state)
}
pub fn verify_treasury_replay(
    data: &TreasuryReplayData,
    expected_root: &str,
) -> Result<(), String> {
    let state = replay_treasury(data)?;
    if state.treasury_root == expected_root {
        Ok(())
    } else {
        Err("treasury replay root mismatch".to_owned())
    }
}
pub fn verify_restore(
    before: &WorldTreasuryRecord,
    restored: &WorldTreasuryRecord,
    compensation: &[CompensationArtifact],
) -> bool {
    before.treasury_root == restored.treasury_root
        && before.balance == restored.balance
        && compensation.iter().all(|p| !p.payment_id.is_empty())
}
pub fn verify_migration(
    before: &WorldTreasuryRecord,
    after: &WorldTreasuryRecord,
    allocations: &[TreasuryGovernanceAction],
) -> bool {
    before.treasury_id == after.treasury_id
        && before.treasury_root == after.treasury_root
        && !allocations.iter().any(|a| a.execution_root.is_empty())
}

pub fn treasury_status(record: &WorldTreasuryRecord) -> serde_json::Value {
    serde_json::json!({"world_id": record.world_id, "treasury_id": record.treasury_id, "treasury_root": record.treasury_root, "balance": record.balance})
}
pub fn treasury_revenue(record: &WorldTreasuryRecord) -> &[RevenueEvent] {
    &record.income_streams
}
pub fn treasury_expenses(record: &WorldTreasuryRecord) -> &[ExpenseEvent] {
    &record.expense_streams
}
pub fn treasury_payouts(payments: &[CompensationArtifact]) -> &[CompensationArtifact] {
    payments
}
pub fn treasury_grants(grants: &[GrantArtifact]) -> &[GrantArtifact] {
    grants
}

pub fn health_metrics(
    record: &WorldTreasuryRecord,
    successful_grants: i128,
    total_grants: i128,
) -> TreasuryHealthMetrics {
    let revenue: Amount = record.income_streams.iter().map(|r| r.amount).sum();
    let expenses: Amount = record.expense_streams.iter().map(|e| e.amount).sum();
    let reserve: Amount = record.balance.values().sum();
    TreasuryHealthMetrics {
        revenue,
        expenses,
        reserve_ratio_bps: if expenses == 0 {
            10_000
        } else {
            reserve * 10_000 / expenses
        },
        contributor_payouts: record
            .expense_streams
            .iter()
            .filter(|e| e.expense_type == ExpenseType::ContributorPayments)
            .map(|e| e.amount)
            .sum(),
        grant_performance_bps: if total_grants == 0 {
            0
        } else {
            successful_grants * 10_000 / total_grants
        },
        revenue_growth_bps: 0,
        treasury_runway_periods: if expenses == 0 {
            reserve
        } else {
            reserve / expenses
        },
        dependency_concentration_bps: 0,
    }
}

fn treasury_root(record: &WorldTreasuryRecord) -> String {
    stable_hash(&[
        "treasury-root",
        &record.world_id,
        &format!("{:?}", record.balance),
        &format!("{:?}", record.income_streams),
        &format!("{:?}", record.expense_streams),
        &format!("{:?}", record.governance_model),
    ])
}
fn stable_hash(parts: &[&str]) -> String {
    let mut h = Sha256::new();
    for p in parts {
        h.update((p.len() as u64).to_be_bytes());
        h.update(p.as_bytes());
    }
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn treasury_replay_reproduces_root_and_balance() {
        let initial =
            WorldTreasuryRecord::new("frontier.evr", TreasuryGovernanceModel::CouncilControlled);
        let revenue = RevenueEvent {
            revenue_id: "rev1".into(),
            world_id: "frontier.evr".into(),
            revenue_type: RevenueType::MarketplaceFees,
            asset: "EAC".into(),
            amount: 1_000,
            payer_id: "player1".into(),
            attribution: "marketplace:skin".into(),
            occurred_at: "2026-06".into(),
        };
        let expense = ExpenseEvent {
            expense_id: "exp1".into(),
            world_id: "frontier.evr".into(),
            expense_type: ExpenseType::ContributorPayments,
            asset: "EAC".into(),
            amount: 250,
            recipient_id: "dev1".into(),
            artifact_id: "capability:housing.evr".into(),
            governance_action_id: "vote1".into(),
            occurred_at: "2026-06".into(),
        };
        let data = TreasuryReplayData {
            initial,
            revenues: vec![revenue],
            expenses: vec![expense],
            grants: vec![],
            compensation: vec![fixed_payment("pay1", "dev1", "artifact1", "EAC", 250)],
            governance: vec![],
        };
        let state = replay_treasury(&data).unwrap();
        assert_eq!(state.balance["EAC"], 750);
        verify_treasury_replay(&data, &state.treasury_root).unwrap();
    }

    #[test]
    fn royalty_and_health_are_deterministic() {
        assert_eq!(royalty_payment(100, 10, 500), 50);
        assert_eq!(revenue_share(2_000, 1250), 250);
    }
}
