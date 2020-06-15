//! # twilight-http
//!
//! HTTP support for the twilight ecosystem.
//!
//! ## Features
//!
//! ### Deserialization
//!
//! `twilight-http` supports `serde_json` and `simd-json` for deserializing
//! responses. `serde_json` is enabled by default.
//!
//! #### `simd-json`
//!
//! The `simd-json` feature enables [`simd-json`] support to use simd features of
//! the modern cpus to deserialize responses faster. It is not enabled by
//! default, and instead the `serde_json` feature is enabled by default.
//!
//! To use this feature you need to also add these lines to
//! `<project root>/.cargo/config`:
//! ```toml
//! [build]
//! rustflags = ["-C", "target-cpu=native"]
//! ```
//!
//! You can also set the environment variable
//! `RUSTFLAGS="-C target-cpu=native"`. If you enable both `serde_json` and
//! `simd-json` at the same time, then `simd-json` will be used.
//!
//! #### `serde_json`
//!
//! `serde_json` is the inverse of `simd-json` and will use the `serde_json`
//! crate to deserialize responses.
//!
//! [`simd-json`]: https://crates.io/crates/simd-json

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::pub_enum_variant_names,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    // This issue made it into a stable clippy:
    //
    // <https://github.com/rust-lang/rust-clippy/issues/5360>
    clippy::used_underscore_binding
)]

pub mod api_error;
pub mod client;
pub mod error;
pub mod ratelimiting;
pub mod request;
pub mod routing;

pub use crate::{
    client::Client,
    error::{Error, Result},
};

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub(crate) fn json_from_slice<'a, T: serde::de::Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    #[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
    return serde_json::from_slice(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_slice(s);
}

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
pub(crate) use serde_json::to_vec as json_to_vec;
#[cfg(feature = "simd-json")]
pub(crate) use simd_json::to_vec as json_to_vec;
