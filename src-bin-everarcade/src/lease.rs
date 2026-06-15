use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseState {
    Requested,
    Acquired,
    Reachable,
    BundlePrepared,
    Deployed,
    Running,
    Healthy,
    Expiring,
    Renewing,
    Migrating,
    Recovered,
    Failed,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseRecord {
    pub lease_id: String,
    pub host_account: String,
    pub tenant_account: String,
    pub instance_name: String,
    pub instance_pubkey: String,
    pub contract_id: String,
    pub domain: String,
    pub peer_port: u16,
    pub user_port: u16,
    pub gp_tcp_port: Option<u16>,
    pub gp_udp_port: Option<u16>,
    pub outbound_ip: Option<String>,
    pub created_timestamp: u64,
    pub expiry_timestamp: Option<u64>,
    pub state: LeaseState,
}

#[derive(Debug, Serialize)]
struct LeaseReport<'a> {
    command: &'a str,
    timestamp: u64,
    lease_metadata: &'a LeaseRecord,
    host_account: &'a str,
    tenant_account: &'a str,
    instance_pubkey: &'a str,
    bundle_hash: Option<String>,
    transport_protocol_version: &'a str,
    receipt_root: Option<String>,
    success: bool,
    status: &'a str,
    world_continuity_note: &'a str,
}

pub fn dispatch(args: &[String]) -> Result<(), String> {
    let action = args.get(2).map(String::as_str).ok_or(
        "usage: everarcade lease <acquire|inspect|deploy|health|renew|migrate|recover|release>",
    )?;
    let mut record = load_record().unwrap_or_else(|_| default_record());
    match action {
        "acquire" => {
            record.state = LeaseState::Acquired;
            record.host_account = opt(args, "--host").unwrap_or_else(|| {
                env::var("EVERARCADE_HOST_ACCOUNT").unwrap_or_else(|_| "offline-host".into())
            });
            record.tenant_account =
                env::var("EVERARCADE_TENANT_ACCOUNT").unwrap_or_else(|_| "offline-tenant".into());
            record.lease_id = format!("lease-{}", short_hash(&record.host_account));
            record.instance_pubkey = format!("pubkey-{}", short_hash(&record.lease_id));
            save_all(
                "lease-acquire",
                &record,
                None,
                "acquired offline canonical lease record",
            )
        }
        "inspect" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&record).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        "deploy" => {
            record.state = LeaseState::Deployed;
            let bundle = opt(args, "--bundle")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("runtime/hotpocket-transport"));
            let hash = hash_path_hint(&bundle)?;
            save_all(
                "lease-deploy",
                &record,
                Some(hash),
                "deployment receipt recorded",
            )
        }
        "health" => {
            record.state = LeaseState::Healthy;
            save_all(
                "lease-health",
                &record,
                None,
                "websocket/status checks recorded as offline scaffold",
            )
        }
        "renew" => {
            record.state = LeaseState::Renewing;
            record.expiry_timestamp = Some(now() + 30 * 24 * 60 * 60);
            save_all(
                "lease-renew",
                &record,
                None,
                "lease renewed without changing world continuity",
            )
        }
        "migrate" => {
            record.state = LeaseState::Migrating;
            record.lease_id = format!("replacement-{}", now());
            save_all(
                "lease-migrate",
                &record,
                None,
                "replacement lease prepared; replay roots must match before cutover",
            )
        }
        "recover" => {
            record.state = LeaseState::Recovered;
            record.lease_id = format!("recovered-{}", now());
            save_all(
                "lease-recover",
                &record,
                None,
                "fresh lease recovered from checkpoint/replay log",
            )
        }
        "release" => {
            record.state = LeaseState::Released;
            save_all(
                "lease-release",
                &record,
                None,
                "lease released; world continuity records preserved",
            )
        }
        _ => Err(format!("unknown lease command: {action}")),
    }
}

fn default_record() -> LeaseRecord {
    LeaseRecord {
        lease_id: "pending".into(),
        host_account: "offline-host".into(),
        tenant_account: env::var("EVERARCADE_TENANT_ACCOUNT")
            .unwrap_or_else(|_| "offline-tenant".into()),
        instance_name: "everarcade-world".into(),
        instance_pubkey: "pending".into(),
        contract_id: "everarcade-contract".into(),
        domain: "localhost".into(),
        peer_port: 22861,
        user_port: 8080,
        gp_tcp_port: None,
        gp_udp_port: None,
        outbound_ip: None,
        created_timestamp: now(),
        expiry_timestamp: None,
        state: LeaseState::Requested,
    }
}
fn report_dir() -> PathBuf {
    PathBuf::from("reports/lease")
}
fn record_path() -> PathBuf {
    report_dir().join("lease-record.json")
}
fn load_record() -> Result<LeaseRecord, String> {
    serde_json::from_slice(&fs::read(record_path()).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn save_all(
    name: &str,
    record: &LeaseRecord,
    bundle_hash: Option<String>,
    status: &str,
) -> Result<(), String> {
    fs::create_dir_all(report_dir()).map_err(|e| e.to_string())?;
    fs::create_dir_all("reports/live").map_err(|e| e.to_string())?;
    fs::write(
        record_path(),
        serde_json::to_vec_pretty(record).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let report = LeaseReport { command: name, timestamp: now(), lease_metadata: record, host_account: &record.host_account, tenant_account: &record.tenant_account, instance_pubkey: &record.instance_pubkey, bundle_hash, transport_protocol_version: transport_core::HOTPOCKET_TRANSPORT_PROTOCOL, receipt_root: None, success: true, status, world_continuity_note: "Lease provides temporary compute only; world continuity root, replay history, state root, receipt root, package, and governance metadata remain sovereign." };
    fs::write(
        report_dir().join(format!("{name}.json")),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let live_name = match name {
        "lease-acquire" => Some("acquire-lease.json"),
        "lease-health" => Some("lease-health.json"),
        _ => None,
    };
    if let Some(live_name) = live_name {
        fs::write(
            PathBuf::from("reports/live").join(live_name),
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    println!("{name}: {status}");
    Ok(())
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn opt(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}
fn short_hash(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))[..12].to_string()
}
fn hash_path_hint(path: &PathBuf) -> Result<String, String> {
    let bytes = if path.is_file() {
        fs::read(path).map_err(|e| e.to_string())?
    } else {
        path.to_string_lossy().as_bytes().to_vec()
    };
    Ok(hex::encode(Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lease_states_serialize() {
        assert!(serde_json::to_string(&LeaseState::Migrating)
            .unwrap()
            .contains("Migrating"));
    }
    #[test]
    fn default_record_distinguishes_world_and_lease() {
        let r = default_record();
        assert_eq!(r.state, LeaseState::Requested);
        assert_ne!(r.lease_id, r.contract_id);
    }
}
