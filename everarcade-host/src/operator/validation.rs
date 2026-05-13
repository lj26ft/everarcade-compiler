use super::config::OperatorConfig;

pub fn validate_config(config: &OperatorConfig) -> Result<(), String> {
    if config.node_name.trim().is_empty() {
        return Err("node_name cannot be empty".into());
    }
    if config.state_path.trim().is_empty() {
        return Err("state_path cannot be empty".into());
    }
    Ok(())
}
