use super::treaty::ExecutionTreaty;

pub fn domain_participates(treaty: &ExecutionTreaty, domain: [u8; 32]) -> bool {
    treaty.participating_domains.contains(&domain)
}
