use std::collections::BTreeMap;

use contract_api::protocol_records::ProtocolRecord;
use serde::{Deserialize, Serialize};

use super::error::{Result, RustrigRuntimeError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedRecord {
    pub category: String,
    pub action: String,
    pub subject: String,
    pub authoritative: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeState {
    pub world: BTreeMap<String, BTreeMap<String, String>>,
    pub entity: BTreeMap<String, BTreeMap<String, String>>,
    pub combat: Vec<String>,
    pub inventory: BTreeMap<String, BTreeMap<String, String>>,
    pub quests: BTreeMap<String, BTreeMap<String, String>>,
    pub dialogue: BTreeMap<String, BTreeMap<String, String>>,
    pub economy: Vec<String>,
    pub ui: Vec<String>,
    pub replay: Vec<String>,
    pub deployment_intents: Vec<String>,
    pub xrpl_intents: Vec<String>,
}

pub struct RecordApplication;

impl RecordApplication {
    pub fn apply(state: &mut AuthoritativeState, record: &ProtocolRecord) -> Result<AppliedRecord> {
        macro_rules! applied {
            ($cat:literal,$r:expr,$auth:expr) => {{
                AppliedRecord {
                    category: $cat.to_string(),
                    action: $r.action.clone(),
                    subject: $r.subject.clone(),
                    authoritative: $auth,
                }
            }};
        }
        match record {
            ProtocolRecord::World(r) => {
                state.world.insert(r.subject.clone(), r.fields.clone());
                Ok(applied!("world", r, true))
            }
            ProtocolRecord::Entity(r) => {
                state.entity.insert(r.subject.clone(), r.fields.clone());
                Ok(applied!("entity", r, true))
            }
            ProtocolRecord::Combat(r) => {
                state.combat.push(format!("{}:{}", r.action, r.subject));
                Ok(applied!("combat", r, true))
            }
            ProtocolRecord::Inventory(r) => {
                state.inventory.insert(
                    format!(
                        "{}:{}",
                        r.subject,
                        r.fields.get("item").cloned().unwrap_or_default()
                    ),
                    r.fields.clone(),
                );
                Ok(applied!("inventory", r, true))
            }
            ProtocolRecord::Quest(r) => {
                state.quests.insert(r.subject.clone(), r.fields.clone());
                Ok(applied!("quest", r, true))
            }
            ProtocolRecord::Dialogue(r) => {
                state.dialogue.insert(r.subject.clone(), r.fields.clone());
                Ok(applied!("dialogue", r, true))
            }
            ProtocolRecord::Economy(r) => {
                state.economy.push(format!(
                    "{}:{}:{}",
                    r.action,
                    r.subject,
                    r.fields.get("amount").cloned().unwrap_or_default()
                ));
                Ok(applied!("economy", r, true))
            }
            ProtocolRecord::Ui(r) => {
                state.ui.push(format!("{}:{}", r.action, r.subject));
                Ok(applied!("ui", r, true))
            }
            ProtocolRecord::Replay(r) => {
                state.replay.push(format!("{}:{}", r.action, r.subject));
                Ok(applied!("replay", r, true))
            }
            ProtocolRecord::DeploymentIntent(r) => {
                state
                    .deployment_intents
                    .push(format!("{}:{}", r.action, r.subject));
                Ok(applied!("deployment-intent", r, false))
            }
            ProtocolRecord::XrplIntent(r) => {
                state
                    .xrpl_intents
                    .push(format!("{}:{}", r.action, r.subject));
                Ok(applied!("xrpl-intent", r, false))
            }
            ProtocolRecord::Deployment(r) => {
                Err(RustrigRuntimeError::AuthorityMutationRejected(format!(
                    "deployment execution record {} is not accepted from Rustrigs",
                    r.subject
                )))
            }
            ProtocolRecord::Xrpl(r) => {
                Err(RustrigRuntimeError::AuthorityMutationRejected(format!(
                    "xrpl execution record {} is not accepted from Rustrigs",
                    r.subject
                )))
            }
        }
    }

    pub fn apply_all(
        state: &mut AuthoritativeState,
        records: &[ProtocolRecord],
    ) -> Result<Vec<AppliedRecord>> {
        records
            .iter()
            .map(|record| Self::apply(state, record))
            .collect()
    }
}
