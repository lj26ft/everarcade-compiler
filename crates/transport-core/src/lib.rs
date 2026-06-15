use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

pub const HOTPOCKET_TRANSPORT_PROTOCOL: &str = "everarcade.transport.hotpocket.v0.1";
pub const RUNTIME_RECEIPT_PROTOCOL: &str = "everarcade.runtime.receipt.v0.1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportSubmission {
    pub transport: String,
    pub lease_id: String,
    pub contract_id: String,
    pub user_public_key: String,
    pub input_hash: String,
    pub nonce: u64,
    pub ledger_seq_no: Option<u64>,
    #[serde(with = "base64_bytes")]
    pub raw_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationEnvelope {
    pub world_id: String,
    pub player_id: String,
    pub mutation_type: String,
    #[serde(with = "base64_bytes")]
    pub payload: Vec<u8>,
    pub submission_hash: String,
    pub source_transport: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportReceipt {
    pub protocol: String,
    pub world_id: String,
    pub input_hash: String,
    pub state_root: String,
    pub replay_root: String,
    pub receipt_root: String,
    pub continuity_root: String,
    pub ledger_seq_no: Option<u64>,
    pub ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeState {
    pub mutations: Vec<MutationRecord>,
    pub sequence: u64,
    pub last_mutation_hash: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationRecord {
    pub sequence: u64,
    pub envelope: MutationEnvelope,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            mutations: vec![],
            sequence: 0,
            last_mutation_hash: None,
        }
    }
}

pub fn canonical_hash<T: Serialize>(value: &T) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    Ok(hex::encode(Sha256::digest(
        canonicalize_value(&value).as_bytes(),
    )))
}
pub fn hash_transport_submission(s: &TransportSubmission) -> Result<String, String> {
    canonical_hash(s)
}
pub fn hash_mutation_envelope(e: &MutationEnvelope) -> Result<String, String> {
    canonical_hash(e)
}
pub fn hash_transport_receipt(r: &TransportReceipt) -> Result<String, String> {
    canonical_hash(r)
}

pub fn canonicalize<T: Serialize>(value: &T) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    Ok(canonicalize_value(&value))
}

fn canonicalize_value(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => serde_json::to_string(s).expect("string serialize"),
        Value::Array(items) => format!(
            "[{}]",
            items
                .iter()
                .map(canonicalize_value)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            format!(
                "{{{}}}",
                keys.into_iter()
                    .map(|k| format!(
                        "{}:{}",
                        serde_json::to_string(k).unwrap(),
                        canonicalize_value(&map[k])
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

pub fn create_transport_submission(
    transport: &str,
    lease_id: &str,
    contract_id: &str,
    user_public_key: &str,
    nonce: u64,
    ledger_seq_no: Option<u64>,
    raw_payload: Vec<u8>,
) -> Result<TransportSubmission, String> {
    let input_hash = canonical_hash(
        &serde_json::json!({"nonce": nonce, "payload": STANDARD.encode(&raw_payload)}),
    )?;
    Ok(TransportSubmission {
        transport: transport.into(),
        lease_id: lease_id.into(),
        contract_id: contract_id.into(),
        user_public_key: user_public_key.into(),
        input_hash,
        nonce,
        ledger_seq_no,
        raw_payload,
    })
}

pub fn submission_to_envelope(
    submission: &TransportSubmission,
) -> Result<MutationEnvelope, String> {
    validate_submission(submission)?;
    let payload: Value = serde_json::from_slice(&submission.raw_payload)
        .map_err(|e| format!("malformed payload json: {e}"))?;
    let obj = payload.as_object().ok_or("payload must be a JSON object")?;
    let required = |k: &str| {
        obj.get(k)
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| format!("payload missing {k}"))
    };
    let body = obj
        .get("payload")
        .cloned()
        .unwrap_or_else(|| Value::Object(Map::new()));
    Ok(MutationEnvelope {
        world_id: required("world_id")?,
        player_id: required("player_id")?,
        mutation_type: required("mutation_type")?,
        payload: canonicalize(&body)?.into_bytes(),
        submission_hash: submission.input_hash.clone(),
        source_transport: submission.transport.clone(),
    })
}

pub fn validate_submission(s: &TransportSubmission) -> Result<(), String> {
    if s.transport.trim().is_empty()
        || s.lease_id.trim().is_empty()
        || s.contract_id.trim().is_empty()
        || s.user_public_key.trim().is_empty()
    {
        return Err("submission has empty canonical identity field".into());
    }
    if s.raw_payload.is_empty() {
        return Err("submission raw_payload is empty".into());
    }
    let expected = canonical_hash(
        &serde_json::json!({"nonce": s.nonce, "payload": STANDARD.encode(&s.raw_payload)}),
    )?;
    if s.input_hash != expected {
        return Err("submission input_hash does not match raw_payload and nonce".into());
    }
    Ok(())
}

pub fn validate_envelope(e: &MutationEnvelope) -> Result<(), String> {
    if e.world_id.is_empty()
        || e.player_id.is_empty()
        || e.mutation_type.is_empty()
        || e.submission_hash.is_empty()
        || e.source_transport.is_empty()
    {
        return Err("mutation envelope missing canonical field".into());
    }
    serde_json::from_slice::<Value>(&e.payload)
        .map_err(|err| format!("mutation payload must be canonical JSON bytes: {err}"))?;
    Ok(())
}

pub fn execute_deterministic_mutation(
    state: &RuntimeState,
    envelope: MutationEnvelope,
) -> Result<RuntimeState, String> {
    validate_envelope(&envelope)?;
    let mut next = state.clone();
    next.sequence += 1;
    let record = MutationRecord {
        sequence: next.sequence,
        envelope,
    };
    next.last_mutation_hash = Some(canonical_hash(&record)?);
    next.mutations.push(record);
    Ok(next)
}

pub fn create_receipt(
    world_id: &str,
    input_hash: &str,
    state: &RuntimeState,
    ledger_seq_no: Option<u64>,
    ok: bool,
) -> Result<TransportReceipt, String> {
    let state_root = canonical_hash(&serde_json::json!({"label":"state", "state": state}))?;
    let replay_root =
        canonical_hash(&serde_json::json!({"label":"replay", "mutations": state.mutations}))?;
    let receipt_root = canonical_hash(
        &serde_json::json!({"label":"receipt", "world_id": world_id, "input_hash": input_hash, "state_root": state_root, "replay_root": replay_root, "ok": ok}),
    )?;
    let continuity_root = canonical_hash(
        &serde_json::json!({"label":"continuity", "receipt_root": receipt_root, "sequence": state.sequence}),
    )?;
    Ok(TransportReceipt {
        protocol: RUNTIME_RECEIPT_PROTOCOL.into(),
        world_id: world_id.into(),
        input_hash: input_hash.into(),
        state_root,
        replay_root,
        receipt_root,
        continuity_root,
        ledger_seq_no,
        ok,
    })
}

pub fn process_accepted_submission(
    submission: &TransportSubmission,
    state: &RuntimeState,
) -> Result<(MutationEnvelope, RuntimeState, TransportReceipt), String> {
    let envelope = submission_to_envelope(submission)?;
    let next_state = execute_deterministic_mutation(state, envelope.clone())?;
    let receipt = create_receipt(
        &envelope.world_id,
        &envelope.submission_hash,
        &next_state,
        submission.ledger_seq_no,
        true,
    )?;
    Ok((envelope, next_state, receipt))
}

mod base64_bytes {
    use super::*;
    pub fn serialize<S>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&STANDARD.encode(bytes))
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(d)?;
        STANDARD.decode(text).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sub() -> TransportSubmission {
        create_transport_submission(HOTPOCKET_TRANSPORT_PROTOCOL, "lease-1", "contract-1", "user-pub", 7, Some(42), br#"{"world_id":"world-1","player_id":"player-1","mutation_type":"join_player","payload":{"x":1}}"#.to_vec()).unwrap()
    }
    #[test]
    fn deterministic_hashes() {
        assert_eq!(
            hash_transport_submission(&sub()).unwrap(),
            hash_transport_submission(&sub()).unwrap()
        );
    }
    #[test]
    fn same_input_same_envelope() {
        assert_eq!(
            submission_to_envelope(&sub()).unwrap(),
            submission_to_envelope(&sub()).unwrap()
        );
    }
    #[test]
    fn malformed_payload_rejected() {
        let bad = create_transport_submission(
            HOTPOCKET_TRANSPORT_PROTOCOL,
            "l",
            "c",
            "u",
            1,
            None,
            br#"{"world_id":"w"}"#.to_vec(),
        )
        .unwrap();
        assert!(submission_to_envelope(&bad)
            .unwrap_err()
            .contains("player_id"));
    }
    #[test]
    fn transport_objects_isolated() {
        let out = serde_json::to_string(
            &process_accepted_submission(&sub(), &RuntimeState::default()).unwrap(),
        )
        .unwrap();
        assert!(!out.contains("ctx"));
        assert!(!out.contains("users"));
    }
    #[test]
    fn root_equivalence() {
        let a = process_accepted_submission(&sub(), &RuntimeState::default()).unwrap();
        let b = process_accepted_submission(&sub(), &RuntimeState::default()).unwrap();
        assert_eq!(a.2.receipt_root, b.2.receipt_root);
        assert_eq!(a.2.replay_root, b.2.replay_root);
        assert_eq!(a.2.continuity_root, b.2.continuity_root);
    }
    #[test]
    fn round_trip_json_future_transport() {
        let s = create_transport_submission(
            "everarcade.transport.future.v1",
            "l",
            "c",
            "u",
            0,
            None,
            br#"{"world_id":"w","player_id":"p","mutation_type":"m"}"#.to_vec(),
        )
        .unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let de: TransportSubmission = serde_json::from_str(&json).unwrap();
        assert_eq!(de.transport, "everarcade.transport.future.v1");
        assert_eq!(
            submission_to_envelope(&de).unwrap().source_transport,
            de.transport
        );
    }
}
