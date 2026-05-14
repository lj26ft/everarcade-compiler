use super::xrpl_testnet_config::XrplTestnetConfig;

#[cfg(feature = "xrpl-live")]
pub fn is_available(_config: &XrplTestnetConfig) -> bool {
    true
}

#[cfg(not(feature = "xrpl-live"))]
pub fn is_available(_config: &XrplTestnetConfig) -> bool {
    false
}
