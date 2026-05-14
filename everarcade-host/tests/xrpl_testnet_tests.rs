use everarcade_host::xrpl::{xrpl_testnet::is_available, xrpl_testnet_config::XrplTestnetConfig};

#[test]
fn xrpl_live_is_optional() {
    let cfg = XrplTestnetConfig {
        endpoint: "https://example".into(),
    };
    let _ = is_available(&cfg);
}
