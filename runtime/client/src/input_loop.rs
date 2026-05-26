use crate::console::{parse_command, ConsoleCommand};
use execution_core::game_runtime::input_runtime::{InputAction, RuntimeInput};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerCommand {
    Runtime(RuntimeInput),
    Save,
    Load,
    Pause,
    Resume,
    Replay,
    Step,
    Status,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputFrame {
    pub sequence: u64,
    pub tick: u64,
    pub command: PlayerCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeInputSession {
    pub session_id: String,
    pub frames: Vec<InputFrame>,
}

#[derive(Debug, Clone)]
pub struct InteractiveInputLoop {
    pub session: RuntimeInputSession,
}

impl InteractiveInputLoop {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session: RuntimeInputSession {
                session_id: session_id.into(),
                frames: Vec::new(),
            },
        }
    }

    pub fn parse_line(&mut self, tick: u64, player: &str, line: &str) -> Option<InputFrame> {
        let command = match parse_command(line)? {
            ConsoleCommand::Move(dir) => {
                let action = match dir.as_str() {
                    "w" | "up" => InputAction::MoveUp,
                    "s" | "down" => InputAction::MoveDown,
                    "a" | "left" => InputAction::MoveLeft,
                    "d" | "right" => InputAction::MoveRight,
                    "inventory" => InputAction::InventoryAction,
                    _ => return None,
                };
                PlayerCommand::Runtime(RuntimeInput {
                    tick,
                    player_id: player.to_string(),
                    action,
                })
            }
            ConsoleCommand::Save => PlayerCommand::Save,
            ConsoleCommand::Load => PlayerCommand::Load,
            ConsoleCommand::Pause => PlayerCommand::Pause,
            ConsoleCommand::Play | ConsoleCommand::Resume => PlayerCommand::Resume,
            ConsoleCommand::Replay => PlayerCommand::Replay,
            ConsoleCommand::Step | ConsoleCommand::Tick => PlayerCommand::Step,
            ConsoleCommand::Status => PlayerCommand::Status,
            ConsoleCommand::Quit => PlayerCommand::Quit,
            ConsoleCommand::Inventory => PlayerCommand::Runtime(RuntimeInput {
                tick,
                player_id: player.to_string(),
                action: InputAction::InventoryAction,
            }),
            _ => return None,
        };
        let frame = InputFrame {
            sequence: self.session.frames.len() as u64,
            tick,
            command,
        };
        self.session.frames.push(frame.clone());
        Some(frame)
    }
}
