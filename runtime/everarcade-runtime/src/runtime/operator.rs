use crate::runtime::*;
use anyhow::{anyhow, Result};
use sha2::Digest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperatorCommand {
    Start,
    Stop,
    Restart,
    Status,
    Verify,
    Backup,
    Restore,
    ReplayVerify,
    ReplayReport,
    ReplayRoot,
    ExecuteProof,
    ExecuteTemplateProof,
    Checkpoint,
    Recover,
    Doctor,
}

impl OperatorCommand {
    pub fn parse(s: &str) -> Result<Self> {
        Ok(match s {
            "start" => Self::Start,
            "stop" => Self::Stop,
            "restart" => Self::Restart,
            "status" => Self::Status,
            "verify" => Self::Verify,
            "backup" => Self::Backup,
            "restore" => Self::Restore,
            "replay-verify" => Self::ReplayVerify,
            "replay-report" => Self::ReplayReport,
            "replay-root" => Self::ReplayRoot,
            "execute-proof" => Self::ExecuteProof,
            "execute-template-proof" => Self::ExecuteTemplateProof,
            "checkpoint" => Self::Checkpoint,
            "recover" => Self::Recover,
            "doctor" => Self::Doctor,
            other => return Err(anyhow!("unknown runtime operator command: {other}")),
        })
    }
}

#[derive(Clone, Debug)]
pub struct RuntimeOperator {
    pub config: RuntimeConfiguration,
}

impl RuntimeOperator {
    pub fn new(config: RuntimeConfiguration) -> Self {
        Self { config }
    }

    pub fn execute(&self, command: OperatorCommand) -> Result<String> {
        match command {
            OperatorCommand::Start => {
                RuntimeLoop::boot(self.config.clone())?;
                Ok("started".into())
            }
            OperatorCommand::Stop => {
                let mut rt = RuntimeLoop::boot(self.config.clone())?;
                rt.stop()?;
                Ok("stopped".into())
            }
            OperatorCommand::Restart => {
                let mut rt = RuntimeLoop::boot(self.config.clone())?;
                rt.stop()?;
                RuntimeLoop::boot(self.config.clone())?;
                Ok("restarted".into())
            }
            OperatorCommand::Status => {
                let p = PersistenceManager::new(&self.config.root);
                let h: RuntimeHealth = p.read_versioned(self.config.runtime_status_path())?;
                Ok(serde_json::to_string_pretty(&h)?)
            }
            OperatorCommand::Verify => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                rt.journal.verify()?;
                Ok("verified".into())
            }
            OperatorCommand::Recover => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                let report = RecoveryManager {
                    checkpoint_manager: rt.checkpoints,
                    journal_manager: rt.journal,
                    replay_manager: ReplayManager,
                }
                .recover()?;
                Ok(serde_json::to_string_pretty(&report)?)
            }
            OperatorCommand::ReplayVerify => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                rt.journal.verify()?;
                Ok("replay verified".into())
            }
            OperatorCommand::ReplayReport => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                let entries = rt.journal.entries()?;
                let report = ReplayManager.report(
                    &[],
                    &entries,
                    entries
                        .last()
                        .map(|e| e.state_root.as_str())
                        .unwrap_or_default(),
                );
                Ok(serde_json::to_string_pretty(&report)?)
            }
            OperatorCommand::ReplayRoot => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                let entries = rt.journal.entries()?;
                Ok(ReplayManager::replay_root(&[], &entries))
            }
            OperatorCommand::ExecuteProof => {
                let mut rt = RuntimeLoop::boot(self.config.clone())?;
                let proof = rt.execute_deterministic_proof()?;
                Ok(serde_json::to_string_pretty(&proof)?)
            }
            OperatorCommand::ExecuteTemplateProof => {
                let mut rt = RuntimeLoop::boot(self.config.clone())?;
                let proof = rt.execute_template_gameplay_proof()?;
                Ok(serde_json::to_string_pretty(&proof)?)
            }
            OperatorCommand::Checkpoint => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                let root = hex::encode(sha2::Sha256::digest(&rt.state));
                rt.checkpoints.create(
                    rt.health.journal_height,
                    &self.config.world_id,
                    &self.config.runtime_version,
                    rt.health.journal_height,
                    root,
                    rt.state,
                    rt.package.world_metadata,
                )?;
                Ok("checkpointed".into())
            }
            OperatorCommand::Backup => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                let manifest = BackupManager {
                    dir: self.config.backups_dir(),
                    checkpoints: rt.checkpoints,
                    persistence: rt.persistence,
                }
                .backup()?;
                Ok(serde_json::to_string_pretty(&manifest)?)
            }
            OperatorCommand::Restore => Ok("restore prepared".into()),
            OperatorCommand::Doctor => {
                let rt = RuntimeLoop::boot(self.config.clone())?;
                rt.journal.verify()?;
                Ok("doctor ok".into())
            }
        }
    }
}
