#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsoleCommand { Move(String), Tick, Save, Load, Restore, Resume, Replay, Verify, Checkpoint, Inventory, Status, Quit }

pub fn parse_command(line: &str) -> Option<ConsoleCommand> {
    let trimmed = line.trim();
    if trimmed == "tick" { return Some(ConsoleCommand::Tick); }
    if trimmed == "save" { return Some(ConsoleCommand::Save); }
    if trimmed == "load" { return Some(ConsoleCommand::Load); }
    if trimmed == "restore" { return Some(ConsoleCommand::Restore); }
    if trimmed == "resume" { return Some(ConsoleCommand::Resume); }
    if trimmed == "replay" { return Some(ConsoleCommand::Replay); }
    if trimmed == "checkpoint" { return Some(ConsoleCommand::Checkpoint); }
    if trimmed == "verify" { return Some(ConsoleCommand::Verify); }
    if trimmed == "inventory" { return Some(ConsoleCommand::Inventory); }
    if trimmed == "status" { return Some(ConsoleCommand::Status); }
    if trimmed == "quit" { return Some(ConsoleCommand::Quit); }
    trimmed.strip_prefix("move ").map(|d| ConsoleCommand::Move(d.to_string()))
}
