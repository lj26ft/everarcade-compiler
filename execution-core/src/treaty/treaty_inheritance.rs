use super::treaty::ExecutionTreaty;

pub fn inherited_domains(parent: &ExecutionTreaty, child_extra: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut out = parent.participating_domains.clone();
    out.extend_from_slice(child_extra);
    out
}
