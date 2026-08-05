//! Mostly a wrapper around [`photon_types`]

use std::sync::LazyLock;

pub use photon_types::*;

pub mod proto;

// Do not make this public outside of this crate, as it could break semver
pub(crate) static VERSION: LazyLock<Version> = LazyLock::new(|| {
    const VERSION_STR: &str = env!("CARGO_PKG_VERSION");
    semver::Version::parse(VERSION_STR).unwrap().into()
});
