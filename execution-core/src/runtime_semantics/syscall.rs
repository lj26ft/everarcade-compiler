#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syscall {
    ReadMemory,
    WriteMemory,
    ExecutionMetadata,
    StateRead,
    StateWrite,
}

pub fn is_allowed(syscall: Syscall) -> bool {
    matches!(
        syscall,
        Syscall::ReadMemory
            | Syscall::WriteMemory
            | Syscall::ExecutionMetadata
            | Syscall::StateRead
            | Syscall::StateWrite
    )
}
