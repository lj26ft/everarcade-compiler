pub fn validate_runtime_compatibility(runtime_version: &str) -> Result<(), &'static str> { if runtime_version.starts_with("everarcade-") { Ok(()) } else { Err("incompatible runtime package") } }
