#![doc = "The package carrying the Topcoat Blocks custom UI registry."]
#![forbid(unsafe_code)]

/// The registry crate name used with `topcoat ui --registry`.
pub const REGISTRY_NAME: &str = env!("CARGO_PKG_NAME");
